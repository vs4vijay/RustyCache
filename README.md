# RustyCache
A Cache implementation in Rust for fun

## Installation

To use RustyCache in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rusty_cache = { git = "https://github.com/vs4vijay/RustyCache.git" }
```

## Usage

Here's a simple example of how to use RustyCache:

```rust
use rusty_cache::Cache;

fn main() {
    let mut cache = Cache::new();
    cache.add(1, "one");
    println!("{:?}", cache.get(&1)); // Output: Some("one")
    cache.remove(&1);
    println!("{:?}", cache.get(&1)); // Output: None
}
```

## Running Tests

To run the tests for RustyCache, use the following command:

```sh
cargo test
```

## Inspiration

This project was inspired by the following repositories:
- [mini-redis](https://github.com/tokio-rs/mini-redis/)
- [dice](https://github.com/dicedb/dice)
