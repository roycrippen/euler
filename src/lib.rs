//! Solutions to selected Project Euler problems. (https://projecteuler.net)
//!
//! # Using this crate
//!
//! Clone this repository, move to euler_soutions directory and build.
//!
//! `...some_path/euler_soutions/cargo build --release`
//!
//!
//! Run all solutions.
//!
//! `...some_path/euler_soutions/cargo run --release`
//!
//! Or run one solution, example run solution 88.
//!
//! `...some_path/euler_soutions/cargo run --release 88`
//!
//!
//! Much faster by moving to release directory and running binary.
//!
//! `...some_path/euler_soutions/target/release/time ./eu_all`
//! `...some_path/euler_soutions/target/release/time ./eu_all 88`
//!
//! result list will roughly be ordered by execution time, slowest last
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

pub mod eu001_010;
pub mod eu011_020;

// Returns a `HashMap` of ("problem number", function to execute) from list of functions fns
fn get_fn_map(fns: &[fn() -> String], start: u32) -> HashMap<String, fn() -> String> {
    fns.iter()
       .enumerate()
       .map(|(i, &f)| ((i as u32 + start).to_string(), f))
       .collect::<HashMap<_, _>>()
}

/// Facilitates execution of one or more solutions stored in function vector fns.
///
/// ```example
/// extern crate euler_solutions;
/// use euler_solutions as sol;
/// use euler_solutions::eu001_010;
///
/// let fns = eu001_010::get_functions();
/// // runs solution 5
/// sol::run(fns, Some("5"), 1);
/// // runs all solutions in fns
/// sol::run(fns, None, 1);
/// ```
pub fn run(fns: Vec<fn() -> String>, arg_maybe: Option<String>, start: u32) {
    let mut fns = fns;
    let fn_map = get_fn_map(&fns, start);
    match arg_maybe {
        Some(arg) => {
            if fn_map.contains_key(&arg) {
                fns = vec![*fn_map.get(&arg).unwrap()];
            }
        }
        _ => (),
    }
    // reversed is faster, gives harder higher number problems more time
    fns.reverse();
    execute(fns)
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
}
