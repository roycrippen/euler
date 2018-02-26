# euler
Solutions to selected [Project Euler](https://projecteuler.net/) problems written in [Rust](https://www.rust-lang.org/).

Please read the [API documentation here](http://roycrippen.github.io/euler_rust/euler/index.html)

[![Build Status](https://travis-ci.org/roycrippen/euler.svg?branch=master)](https://travis-ci.org/roycrippen/euler)

## Using this crate

Just add the following to your [`Cargo.toml`](http://crates.io/):
```
[dependencies.euler]
git = "https://github.com/roycrippen/euler"
```

And add this to your root crate.
```rust
extern crate euler;
```

## Example

```rust
extern crate euler;

fn main() {
    // test solution 16
    assert_eq!(euler_rust::euler_p011_020::p016(), "p016 = 1366");

    // run all solutions concurrently
    euler_rust::run_all();
}
```

### Build

```rust
cargo build --release
```

### Run

run solution to problem 120

```rust
cargo run --release 120
```

run all solutions concurrently

```rust
cargo run --release
```

Result list will be ordered by execution time, slowest last.


