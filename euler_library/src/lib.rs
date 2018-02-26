// #![feature(plugin)]
// #![plugin(clippy)]

//!  Several libraries used in solving Project Euler problems (https://projecteuler.net)
//!
//! # Using this library
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies.euler_library]
//! git = "https://github.com/roycrippen/euler_library"
//! ```
//! And add this to your root crate.
//!
//! ```
//! extern crate euler_library;
//! ```
//!
pub mod big;
pub mod cards;
pub mod common;
pub mod primes;
