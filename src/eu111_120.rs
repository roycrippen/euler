//! Project Euler solutions for problems 111 through 120.

use std::iter::repeat;
use std::cmp;
use std::usize;

extern crate num;
use self::num::BigUint;
use self::num::bigint::ToBigUint;

extern crate primal;

extern crate permutohedron;
use self::permutohedron::LexicalPermutation;

extern crate euler_library;
use self::euler_library::common as eu;
use self::euler_library::big as eu_big;

/// Primes with runs
pub fn eu111() -> String {
    fn from_digits(xs: &Vec<usize>) -> usize {
        let mut n = 0;
        for x in xs {
            n = n * 10 + x
        }
        n
    }

    fn solve_row(n: usize, d: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let ds = repeat(d).take(n).collect::<Vec<usize>>();
        for i in 1..n {
            if res.len() > 0 {
                return res;
            }
            for canidate in populate(i, &ds) {
                let num = from_digits(&canidate);
                if primal::is_prime(num as u64) {
                    res.push(num)
                }
            }
        }
        return res;
    }

    fn populate(cols: usize, ds: &Vec<usize>) -> Vec<Vec<usize>> {
        let ranges = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut res = Vec::new();
        for i in 0..ds.len() {
            for j in 0..10 {
                if ds[i] == ranges[j] {
                    continue;
                }
                let mut ts = ds.clone();
                ts[i] = ranges[j];
                if ts[0] == 0 {
                    continue;
                }
                res.push(ts.clone());
                if cols == 1 {
                    continue;
                }
                res.append(&mut populate(cols - 1, &ts));
            }
        }
        res.sort();
        res.dedup();
        return res;
    }

    fn solve(n: usize) -> usize {
        let xs = (0..10).map(|i| solve_row(n, i)).collect::<Vec<Vec<usize>>>();

        // for (i, v) in xs.clone().iter().enumerate() {
        //     println!("N({}, {:?})", i, v.len());
        // }

        xs.into_iter().fold(0, |acc, list| acc + list.iter().fold(0, |sum, x| sum + x))
    }

    let res = solve(10);
    assert_eq!(res, 612407567715);
    format!("eu111 = {}", res)
} // 612407567715

/// Bouncy numbers
pub fn eu112() -> String {
    fn bouncy(mut n: usize) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        let mut last = n % 10;
        n /= 10;
        while n > 0 {
            let next = n % 10;
            if next < last {
                increasing = false;
            }
            if next > last {
                decreasing = false;
            }
            if !increasing && !decreasing {
                return true;
            }
            last = next;
            n /= 10;
        }
        return !increasing && !decreasing;
    }

    fn solve(proportion: usize) -> usize {
        let mut cnt = 0;
        for i in 1.. {
            if bouncy(i) {
                cnt += 1
            }
            // let p = (cnt as f64) / (i as f64);
            if 100 * cnt == proportion * i {
                return i;
            }
        }
        0
    }

    let res = solve(99);
    assert_eq!(res, 1587000);
    format!("eu112 = {}", res)
} // 1587000

/// Non-bouncy numbers
pub fn eu113() -> String {
    fn c_nr(n: usize, r: usize) -> BigUint {
        let n_fact = eu_big::factorial(n);
        let r_fact = eu_big::factorial(r);
        let n_minus_r_fact = eu_big::factorial(n - r);
        let res = (n_fact / r_fact) / n_minus_r_fact;
        res
    }

    fn solve(n: usize) -> String {
        let res = c_nr(n + 9, n) + c_nr(n + 10, n) - (10 * n + 2).to_biguint().unwrap();
        res.to_string()
    }

    let res = solve(100).parse::<usize>().unwrap();
    assert_eq!(res, 51161058134250);
    format!("eu113 = {}", res)
} // 51161058134250


/// Counting block combinations I
pub fn eu114() -> String {
    // n=total block length, m=min color length
    // return combination count for all color length from m to n
    fn count_blocks(n: usize, m: usize) -> usize {
        let mut ways = vec![1 as usize];
        for i in 1..n + 1 {
            let mut sum = ways[i - 1];
            let idx = if m > i { 0 } else { i - m };
            for j in 0..idx {
                sum += ways[j];
            }
            if i >= m {
                sum += 1;
            }
            ways.push(sum);
        }
        *ways.last().unwrap()
    }

    let res = count_blocks(50, 3);
    assert_eq!(res, 16475640049);
    format!("eu114 = {}", res)
} // 16475640049

/// Counting block combinations II
pub fn eu115() -> String {
    // n=total block length, m:color length
    // return combination count for a single length m
    fn count_blocks(m: usize, n: usize, ways: &mut Vec<usize>) -> usize {
        let mut sum = ways[n - 1];
        let idx = if m > n { 0 } else { n - m };
        for j in 0..idx {
            sum += ways[j];
        }
        if n >= m {
            sum += 1;
        }
        ways.push(sum);
        *ways.last().unwrap()
    }

    fn solve(m: usize, max: usize) -> usize {
        let mut ways = vec![1];
        for i in 1.. {
            if count_blocks(m, i, &mut ways) > max {
                return i;
            }
        }
        0
    }

    let res = solve(50, 1_000_000);
    assert_eq!(res, 168);
    format!("eu115 = {}", res)
} // 168

/// Red, green or blue tiles
pub fn eu116() -> String {
    // n=total block length, m:color length
    // return combination count for a single length m
    fn count_blocks(n: usize, m: usize) -> usize {
        let mut xs = vec![0; n + 1];
        xs[0] = 1;
        for i in 1..n + 1 {
            xs[i] += xs[i - 1];
            if i >= m {
                xs[i] += xs[i - m];
            }
        }
        xs[n] - 1
    }

    let res = count_blocks(50, 4) + count_blocks(50, 3) + count_blocks(50, 2);
    assert_eq!(res, 20492570929);
    format!("eu116 = {}", res)
} // 20492570929

/// Red, green, and blue tiles
pub fn eu117() -> String {
    // n=total block length, m:color length
    // return combination count for a single length m
    fn solve(n: usize) -> usize {
        let mut xs = vec![0; n + 1];
        xs[0] = 1;
        for i in 1..n + 1 {
            let start = cmp::max((i as i32 - 4), 0) as usize;
            let sub = xs.clone();
            let sub = &sub[start..];
            xs[i] += sub.iter().fold(0, |acc, x| acc + x);
        }
        xs[n]
    }

    let res = solve(50);
    assert_eq!(res, 100808458960497);
    format!("eu117 = {}", res)
} // 100808458960497

/// Pandigital prime sets
pub fn eu118() -> String {
    fn count_prime_sets(ds: &[usize], cur: usize, idx: usize, p: &primal::Sieve) -> usize {
        if idx == 9 {
            return 1;
        }
        let mut n = 0;
        let mut ans = 0;
        for i in idx..9 {
            n = 10 * n + ds[i];
            if n > cur && p.is_prime(n) {
                ans += count_prime_sets(ds, n, i + 1, p);
            }
        }
        ans
    }

    fn solve() -> usize {
        let sieve = primal::Sieve::new(100_000_000);
        let mut sum = 0;
        let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        loop {
            if digits[8] % 2 != 0 && digits[8] != 5 {
                sum += count_prime_sets(&digits, 0, 0, &sieve)
            }
            if !digits.next_permutation() {
                break;
            }
        }
        sum
    }

    let res = solve();
    assert_eq!(res, 44680);
    format!("eu118 = {}", res)
} // 44680

/// Digit power sum
pub fn eu119() -> String {
    fn sum_of_digits(n: usize) -> usize {
        eu::to_bytes(n).into_iter().fold(0, |acc, x| acc + x as usize - 48)
    }

    fn get_exp(n: usize) -> Option<u32> {
        let sod = sum_of_digits(n);
        let exp = ((n as f64).log10() / (sod as f64).log10()).round() as u32;
        if sod.pow(exp) == n { return Some(exp) } else { return None }
    }

    fn make_table() -> Vec<usize> {
        let max = usize::MAX / 10;
        let mut res = Vec::new();
        for i in 2..15 {
            for x in 1..150 {
                let y = i as u32;
                let candidate = (x as usize).pow(y);
                if candidate > 10 && get_exp(candidate) != None {
                    res.push(candidate)
                }
                if candidate > max {
                    break;
                }
            }
        }
        res.sort();
        res.dedup();
        res
    }

    let res = make_table()[29] as usize;
    assert_eq!(res, 248155780267521);
    format!("eu119 = {}", res)
} // 248155780267521

/// Square remainders
pub fn eu120() -> String {
    // https://benpyeh.com/2013/06/23/project-euler-120/
    let res = (3..1001).fold(0,
                             |acc, a| if a % 2 == 0 { acc + a * a - 2 * a } else { acc + a * a - a });
    assert_eq!(res, 333082500);
    format!("eu120 = {}", res)
} // 333082500

/// Returns Vec of the Euler solution functions in this crate.
pub fn get_functions() -> Vec<fn() -> String> {
    vec![eu111, eu112, eu113, eu114, eu115, eu116, eu117, eu118, eu119, eu120]
}
