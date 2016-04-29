# Euler Solutions

Solutions to selected [Project Euler](https://projecteuler.net/) problems written in [Rust](https://www.rust-lang.org/).

Please read the [API documentation here](http://roycrippen.github.io/euler_solutions/euler_solutions/index.html)

[![Build Status](https://travis-ci.org/roycrippen/euler_solutions.svg?branch=master)](https://travis-ci.org/roycrippen/euler_solutions)

## Using this crate

Clone this repository, move to euler_soutions directory and build.

```
...some_path/euler_soutions/cargo build --release
```

Run all solutions.

```
...some_path/euler_soutions/cargo run --release
```

Or run one solution, example run solution 88.

```
...some_path/euler_soutions/cargo run --release 88
```

Much faster by moving to release directory and running binary.

```
...some_path/euler_soutions/target/release/time ./eu_all
...some_path/euler_soutions/target/release/time ./eu_all 88
```

result list will roughly be ordered by execution time, slowest last


