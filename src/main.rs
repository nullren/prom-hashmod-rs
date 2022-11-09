use std::io;
use std::io::BufRead;
use clap::Parser;
use prom_hashmod_rs::hashmod;

/// hashmod is a function used by prometheus to hash scrape targets to a specific shard
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Modulus used in the hashmod function
    #[clap(short, long, value_parser, default_value = "100")]
    modulus: u64,
}

fn main() {
    let args = Cli::parse();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let result = hashmod(line.as_bytes(), args.modulus);
        println!("{} {}", result, line);
    }
}
