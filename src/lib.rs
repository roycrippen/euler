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
//! use euler::euler_p011_020 as p11_to_20;
//!
//! fn main() {
//!     // test solution 16
//!     assert_eq!(p11_to_20::p016(), "p016 = 1366");
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
//! run solution to problem 88
//!
//! ```ignore
//! cargo run --release 88
//! ```
//!
//! run all solutions concurrently
//!
//! ```ignore
//! cargo run --release
//! ```
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::env;

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

/// Executes one or more solutions stored in function vector fns.
///
/// ```
/// use std::env;
///
/// extern crate euler;
///
/// // Invalid or no runtime argument executes p011 through p020 concurrently.
/// // Executes solution given by single valid runtime argument
/// // In this example vaild args are integers 11 through 20
/// fn main() {
///     let (start, fns) = euler::euler_p011_020::get_functions();
///     euler::run(fns, env::args().nth(1), start);
/// }
/// ```
pub fn run(fns: Vec<fn() -> String>, arg_maybe: Option<String>, start: u32) {
    let mut fns = fns;
    let fn_map = get_fn_map(&fns, start);
    if let Some(arg) = arg_maybe {
        if fn_map.contains_key(&arg) {
            fns = vec![*fn_map.get(&arg).unwrap()];
        }
    }
    // reversed is faster, gives harder higher number problems more time
    fns.reverse();
    execute(fns)
}

/// Executes one or all solutions from `euler` crate.
///
/// ```
/// extern crate euler;
///
/// // Invalid or no runtime argument executes all solutions concurrently.
/// // Executes solution given by single valid runtime argument.
/// // In this example vaild args are integers 1 through solved solutions.
/// fn main() {
///     euler::run_all();
/// }
/// ```
pub fn run_all() {
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

    run(fns.clone(), env::args().nth(1), 1);

}

// Returns a `HashMap` of ("problem number", function to execute) from list of functions fns
fn get_fn_map(fns: &[fn() -> String], start: u32) -> HashMap<String, fn() -> String> {
    fns.iter()
       .enumerate()
       .map(|(i, &f)| ((i as u32 + start).to_string(), f))
       .collect::<HashMap<_, _>>()
}

// Executes all functions in fns concurrently
fn execute(fns: Vec<fn() -> String>) {
    let (tx, rx) = mpsc::channel();
    for f in fns.clone() {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(f()).expect("channel send to euler function failed");
        });
    }

    for _ in 0..fns.len() {
        println!("{}", rx.recv().expect("channel receive failed"));
    }
    if fns.len() > 1 {
        println!("\nSuccessfully solved {} Project Euler problems.",
                 fns.len());
    }
}
