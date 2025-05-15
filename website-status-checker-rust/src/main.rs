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
    period: Option<u64>,
    assert_header: Option<(String, String)>,
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

    loop {
        let results = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = mpsc::channel::<String>();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..config.workers {
            let rx = Arc::clone(&rx);
            let client = client.clone();
            let results = Arc::clone(&results);
            let retries = config.retries;
            let assert_header = config.assert_header.clone();

            thread::spawn(move || {
                while let Ok(url) = rx.lock().unwrap().recv() {
                    let status = check_url(&client, &url, retries, &assert_header);
                    println!("{}", format_status(&status));
                    results.lock().unwrap().push(status);
                }
            });
        }

        for url in &urls {
            tx.send(url.clone()).unwrap();
        }

        drop(tx);

        while Arc::strong_count(&results) > 1 {
            thread::sleep(Duration::from_millis(100));
        }

        let results_guard = results.lock().unwrap();
        write_json("status.json", &results_guard).expect("Failed to write status.json");
        print_summary_stats(&results_guard);

        if let Some(period) = config.period {
            thread::sleep(Duration::from_secs(period));
        } else {
            break;
        }
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut file = None;
    let mut urls = Vec::new();
    let mut workers = num_cpus::get();
    let mut timeout = 5;
    let mut retries = 0;
    let mut period = None;
    let mut assert_header = None;

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
            "--period" => {
                i += 1;
                period = Some(args[i].parse().map_err(|_| "Invalid number for --period")?);
            }
            "--assert-header" => {
                i += 1;
                if i >= args.len() || !args[i].contains('=') {
                    return Err("Expected format: --assert-header Header=Value".into());
                }
                let parts: Vec<&str> = args[i].splitn(2, '=').collect();
                assert_header = Some((parts[0].to_string(), parts[1].to_string()));
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
        period,
        assert_header,
    })
}

fn print_usage() {
    eprintln!(
        "Usage:\n    website_checker [--file sites.txt] [URL ...]\n                    [--workers N] [--timeout S] [--retries N]\n                    [--period S] [--assert-header Header=Value]"
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

fn check_url(client: &Client, url: &str, retries: usize, assert_header: &Option<(String, String)>) -> WebsiteStatus {
    let start = Instant::now();
    let mut attempts = 0;

    loop {
        let now = SystemTime::now();
        match client.get(url).send() {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let passed = if let Some((header, expected)) = assert_header {
                    match resp.headers().get(header) {
                        Some(val) if val.to_str().unwrap_or("") == expected => Ok(code),
                        _ => Err(format!("Header '{}' mismatch or missing", header)),
                    }
                } else {
                    Ok(code)
                };

                return WebsiteStatus {
                    url: url.to_string(),
                    status: passed,
                    response_time: start.elapsed(),
                    timestamp: now,
                };
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

fn print_summary_stats(statuses: &[WebsiteStatus]) {
    let times: Vec<_> = statuses
        .iter()
        .filter(|s| s.status.is_ok())
        .map(|s| s.response_time.as_millis())
        .collect();

    if times.is_empty() {
        println!("No successful responses.");
        return;
    }

    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    let avg = times.iter().sum::<u128>() as f64 / times.len() as f64;

    println!(
        "\nSummary: min = {} ms, max = {} ms, avg = {:.2} ms\n",
        min, max, avg
    );
}
