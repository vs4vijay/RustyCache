use clap::Parser;
use rusty_cache::Cache;

#[derive(Parser)]
struct Cli {
    key: String,
    value: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let mut cache = Cache::new();

    if let Some(value) = args.value {
        cache.add(args.key, value);
        println!("Added key-value pair to cache.");
    } else {
        match cache.get(&args.key) {
            Some(value) => println!("Value: {}", value),
            None => println!("Key not found in cache."),
        }
    }
}
