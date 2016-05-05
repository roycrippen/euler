# Euler Solutions

Solutions to selected [Project Euler](https://projecteuler.net/) problems written in [Rust](https://www.rust-lang.org/).

Please read the [API documentation here](http://roycrippen.github.io/euler_solutions/euler_solutions/index.html)

[![Build Status](https://travis-ci.org/roycrippen/euler_solutions.svg?branch=master)](https://travis-ci.org/roycrippen/euler_solutions)

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

# Example

```rust
extern crate euler;

// Runs all solutions concurrently.
euler::run_all();

// test solution 16
! assert_eq!(euler::euler_p011_020::p016(), "p016 = 1366");
```

Result list will roughly be ordered by execution time, slowest last.


