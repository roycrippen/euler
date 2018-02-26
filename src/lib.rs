// #![feature(plugin)]
// #![plugin(clippy)]

//! Aggregator crate of solutions to Project Euler problems. (https://projecteuler.net)
//!
//!
//! # Using this crate
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies.euler]
//! git = "https://github.com/roycrippen/euler"
//! ```
//! And add this to your root crate.
//!
//! ```
//! extern crate euler;
//! ```
//!
//! ## Example
//!
//! ```
//! extern crate euler;
//!
//! fn main() {
//!     // test solution 16
//!     assert_eq!(euler::euler_p011_020::p016(), "p016 = 1366");
//!
//!     // run all solutions concurrently
//!     euler::run_all();
//! }
//! ```
//!
//! ## Build
//!
//! ```ignore
//! cargo build --release
//! ```
//!
//! ## Run
//!
//! run solution to problem 120
//!
//! ```ignore
//! cargo run --release 120
//! ```
//!
//! run all solutions concurrently
//!
//! ```ignore
//! cargo run --release
//! ```
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

pub extern crate euler_library;
pub extern crate euler_p001_010;
pub extern crate euler_p011_020;
pub extern crate euler_p021_030;
pub extern crate euler_p031_040;
pub extern crate euler_p041_050;
pub extern crate euler_p051_060;
pub extern crate euler_p061_070;
pub extern crate euler_p071_080;
pub extern crate euler_p081_090;
pub extern crate euler_p091_100;
pub extern crate euler_p101_110;
pub extern crate euler_p111_120;
pub extern crate euler_p121_130;

extern crate rayon;

/// Executes one or more solutions stored in function vector fns.
///
/// ```
/// use std::env;
///
/// extern crate euler;
///
/// // Invalid or no runtime argument executes p011 through p020 concurrently.
/// // Executes solution given by single valid runtime argument
/// // In this example valid args are integers 11 through 20
/// fn main() {
///     let (start, fns) = euler::euler_p011_020::get_functions();
///     euler::run(fns, env::args().nth(1), start);
/// }
/// ```
pub fn run(fns: Vec<fn() -> String>, arg_maybe: Option<String>, start: u32) {
  if let Some(arg) = arg_maybe {
    let fn_map = get_fn_map(&fns, start);
    if fn_map.contains_key(&arg) {
      let f = vec![*fn_map.get(&arg).unwrap()];
      let (res, t) = execute_par_iter(f)[0].clone();
      println!("{:25}, time = {}", res, t);
    } else {
      println!("invalid argument: {}", arg);
      println!("valid argument: a number between 1 and {}", fns.len())
    }
    return;
  }
  println!("Solving {} Euler functions in parallel\n", fns.len());
  let instant = Instant::now();
  let xs = execute_par_iter(fns);
  for (res, t) in xs.clone() {
    println!("{:25}, time = {:.6} s", res, t)
  }
  let duration = get_duration(instant.elapsed());
  println!("\n     total elapsed time: {:.6} s", duration);

  let sum_exec = xs.iter().fold(0.0, |acc, x| acc + x.1);
  println!(" sum of execution times: {:.6} s", sum_exec);

  let par_fact = sum_exec / duration;
  println!("parallel speedup factor: {:.3}", par_fact);
}

/// Executes one or all solutions from `euler` crate.
///
/// ```
/// extern crate euler;
///
/// // Invalid or no runtime argument executes all solutions concurrently.
/// // Executes solution given by single valid runtime argument.
/// // In this example valid args are integers 1 through solved solutions.
/// fn main() {
///     euler::run_all();
/// }
/// ```
pub fn run_all() {
  run(get_all_functions().clone(), env::args().nth(1), 1);
}

// Returns a Vector of all euler functions
pub fn get_all_functions() -> Vec<fn() -> String> {
  let (_, mut fns) = euler_p001_010::get_functions();
  fns.append(&mut euler_p011_020::get_functions().1);
  fns.append(&mut euler_p021_030::get_functions().1);
  fns.append(&mut euler_p031_040::get_functions().1);
  fns.append(&mut euler_p041_050::get_functions().1);
  fns.append(&mut euler_p051_060::get_functions().1);
  fns.append(&mut euler_p061_070::get_functions().1);
  fns.append(&mut euler_p071_080::get_functions().1);
  fns.append(&mut euler_p081_090::get_functions().1);
  fns.append(&mut euler_p091_100::get_functions().1);
  fns.append(&mut euler_p101_110::get_functions().1);
  fns.append(&mut euler_p111_120::get_functions().1);
  fns.append(&mut euler_p121_130::get_functions().1);

  fns
}

// Returns a `HashMap` of ("problem number", function to execute) from list of functions fns
fn get_fn_map(fns: &[fn() -> String], start: u32) -> HashMap<String, fn() -> String> {
  fns
    .iter()
    .enumerate()
    .map(|(i, &f)| ((i as u32 + start).to_string(), f))
    .collect::<HashMap<_, _>>()
}

// execute all problems in parallel
fn execute_par_iter(fns: Vec<fn() -> String>) -> Vec<(String, f64)> {
  let mut xs: Vec<(String, f64)> = fns
    .par_iter()
    .map(|f| {
      let instant = Instant::now();
      let s = f();
      let elapsed = instant.elapsed();
      (s, get_duration(elapsed))
    })
    .collect();

  // sort by time taken to execute
  xs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  xs
}

// Return duration string in fractional seconds
fn get_duration(dur: Duration) -> f64 {
  let micros = dur.as_secs() * 1_000_000 + dur.subsec_nanos() as u64 / 1_000;
  // let frac_time = micros as f64 / 1_000_000.0;
  // format!("{:.6} s", frac_time)
  micros as f64 / 1_000_000.0
}
