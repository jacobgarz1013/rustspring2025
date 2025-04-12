//Could not find a free sp500 api so I used Solana price instead.

use serde::Deserialize;
use std::{fs::File, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

#[derive(Deserialize, Debug)]
struct BitcoinApiResponse {
    bitcoin: CoinPrice,
}

#[derive(Deserialize, Debug)]
struct CoinPrice {
    usd: f64,
}

struct Bitcoin {
    price: f64,
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url).call().unwrap().into_string().unwrap();
        let parsed: BitcoinApiResponse = serde_json::from_str(&response).unwrap();
        self.price = parsed.bitcoin.usd;
    }

    fn save_to_file(&self) {
        let mut file = File::create("bitcoin_price.txt").unwrap();
        writeln!(file, "Bitcoin Price: ${}", self.price).unwrap();
    }
}

#[derive(Deserialize, Debug)]
struct EthereumApiResponse {
    ethereum: CoinPrice,
}

struct Ethereum {
    price: f64,
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url).call().unwrap().into_string().unwrap();
        let parsed: EthereumApiResponse = serde_json::from_str(&response).unwrap();
        self.price = parsed.ethereum.usd;
    }

    fn save_to_file(&self) {
        let mut file = File::create("ethereum_price.txt").unwrap();
        writeln!(file, "Ethereum Price: ${}", self.price).unwrap();
    }
}

#[derive(Deserialize, Debug)]
struct SolanaApiResponse {
    solana: CoinPrice,
}

struct Solana {
    price: f64,
}

impl Pricing for Solana {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd";
        let response = ureq::get(url).call().unwrap().into_string().unwrap();
        let parsed: SolanaApiResponse = serde_json::from_str(&response).unwrap();
        self.price = parsed.solana.usd;
    }

    fn save_to_file(&self) {
        let mut file = File::create("solana_price.txt").unwrap();
        writeln!(file, "Solana Price: ${}", self.price).unwrap();
    }
}


fn main() {
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(Solana { price: 0.0 }),
    ];

    loop {
        for asset in assets.iter_mut() {
            asset.fetch_price();
            asset.save_to_file();
        }
        println!("The prices have been refreshed. Next update in 10 seconds...");
        thread::sleep(Duration::from_secs(10));	
    }
}
