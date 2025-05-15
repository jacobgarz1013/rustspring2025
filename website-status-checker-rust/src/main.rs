use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime},
};

use reqwest::blocking::Client;

#[derive(Debug)]
struct Config {
    file: Option<String>,
    urls: Vec<String>,
    workers: usize,
    timeout: u64,
    retries: usize,
}

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn main() {
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {e}");
            print_usage();
            std::process::exit(2);
        }
    };

    let urls = match load_urls(&config) {
        Ok(urls) if !urls.is_empty() => urls,
        _ => {
            eprintln!("No URLs provided.");
            print_usage();
            std::process::exit(2);
        }
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build()
        .expect("Failed to create HTTP client");

    let results = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..config.workers {
        let rx = Arc::clone(&rx);
        let client = client.clone();
        let results = Arc::clone(&results);
        let retries = config.retries;

        thread::spawn(move || {
            while let Ok(url) = rx.lock().unwrap().recv() {
                let status = check_url(&client, &url, retries);
                println!("{}", format_status(&status));
                results.lock().unwrap().push(status);
            }
        });
    }

    for url in urls {
        tx.send(url).unwrap();
    }

    drop(tx);

    while Arc::strong_count(&results) > 1 {
        thread::sleep(Duration::from_millis(100));
    }

    write_json("status.json", &results.lock().unwrap()).expect("Failed to write status.json");
}


fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut file = None;
    let mut urls = Vec::new();
    let mut workers = num_cpus::get();
    let mut timeout = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing file path after --file".into());
                }
                file = Some(args[i].clone());
            }
            "--workers" => {
                i += 1;
                workers = args[i].parse().map_err(|_| "Invalid number for --workers")?;
            }
            "--timeout" => {
                i += 1;
                timeout = args[i].parse().map_err(|_| "Invalid number for --timeout")?;
            }
            "--retries" => {
                i += 1;
                retries = args[i].parse().map_err(|_| "Invalid number for --retries")?;
            }
            s if s.starts_with("--") => return Err(format!("Unknown flag: {s}")),
            s => urls.push(s.to_string()),
        }
        i += 1;
    }

    if file.is_none() && urls.is_empty() {
        return Err("No input URLs provided".into());
    }

    Ok(Config {
        file,
        urls,
        workers,
        timeout,
        retries,
    })
}

fn print_usage() {
    eprintln!(
        "Usage:
    website_checker [--file sites.txt] [URL ...]
                    [--workers N] [--timeout S] [--retries N]"
    );
}


fn load_urls(config: &Config) -> io::Result<Vec<String>> {
    let mut urls = Vec::new();

    if let Some(ref path) = config.file {
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                urls.push(trimmed.to_string());
            }
        }
    }

    urls.extend(config.urls.clone());
    Ok(urls)
}


fn check_url(client: &Client, url: &str, retries: usize) -> WebsiteStatus {
    let start = Instant::now();
    let mut attempts = 0;

    loop {
        let now = SystemTime::now();
        match client.get(url).send() {
            Ok(resp) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: Ok(resp.status().as_u16()),
                    response_time: start.elapsed(),
                    timestamp: now,
                }
            }
            Err(e) => {
                if attempts < retries {
                    attempts += 1;
                    thread::sleep(Duration::from_millis(100));
                } else {
                    return WebsiteStatus {
                        url: url.to_string(),
                        status: Err(e.to_string()),
                        response_time: start.elapsed(),
                        timestamp: now,
                    };
                }
            }
        }
    }
}

fn format_status(status: &WebsiteStatus) -> String {
    let status_str = match &status.status {
        Ok(code) => format!("HTTP {}", code),
        Err(err) => format!("ERROR {}", err),
    };

    format!(
        "[{}] {} - {} ms - {}",
        status.timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        status.url,
        status.response_time.as_millis(),
        status_str
    )
}


fn write_json(path: &str, statuses: &[WebsiteStatus]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "[")?;

    for (i, status) in statuses.iter().enumerate() {
        let json = format!(
            "  {{\"url\": \"{}\", \"status\": {}, \"response_time_ms\": {}, \"timestamp\": {}}}",
            status.url,
            match &status.status {
                Ok(code) => code.to_string(),
                Err(err) => format!("\"{}\"", escape_json(err)),
            },
            status.response_time.as_millis(),
            status.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        );
        writeln!(file, "{}{}", json, if i + 1 < statuses.len() { "," } else { "" })?;
    }

    writeln!(file, "]")?;
    Ok(())
}

fn escape_json(s: &str) -> String {
    s.replace('"', "\\\"").replace('\n', "\\n")
}
