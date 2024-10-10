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

## Using RustyCache as a Standalone Command

You can also use RustyCache as a standalone command-line tool.

### Installation

To install RustyCache as a standalone command, run the following command:

```sh
cargo install --path .
```

### Example Usage

Here are some examples of how to use RustyCache as a standalone command:

- Add a key-value pair to the cache:

```sh
rusty_cache --key my_key --value my_value
```

- Get the value associated with a key:

```sh
rusty_cache --key my_key
```

- Remove a key from the cache:

```sh
rusty_cache --key my_key --remove
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
