use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    SID: u32,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let SID = lines.next().unwrap().parse().unwrap();

        Config { name, SID }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("name: {}", config.name);
    println!("SID: {}", config.SID);
}

fn main() {
    reading_from_file();
}