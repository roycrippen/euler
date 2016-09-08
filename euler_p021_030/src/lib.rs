//! Project Euler solutions for problems 21 through 30.
//!
//! This crate is designed to be used via crate `euler`.

use std::str::FromStr;
use std::mem;
use std::collections::HashSet;

extern crate primal;

extern crate itertools;
use itertools::Itertools;

extern crate num;
use num::{BigUint, One, Zero};
use num::bigint::ToBigUint;

extern crate euler_library;
use euler_library::common as eu;
use euler_library::big as eu_big;


/// Amicable numbers
pub fn p021() -> String {
    const N: usize = 10_000;

    let amic = eu::divisor_sum_list(N);
    let mut sum = 0;
    for (i, item) in amic.clone().into_iter().enumerate().take(N) {
        if item < N && i != item && i == amic[item] {
            sum += item + amic[item]
        }
    }

    assert_eq!(sum / 2, 31626);
    format!("p021 = {}", sum / 2)
} // 31626

/// Names scores
pub fn p022() -> String {
    let buffer = include_str!("../data/p022_names.txt")
        .chars()
        .filter(|&x| x != '\"' && x != '\n')
        .collect::<String>();

    let names = buffer.split(',')
        .map(|x| {
            let ys = eu::to_bytes(&x);
            ys.iter().map(|&y| y as usize - 64).collect::<Vec<usize>>()
        })
        .sorted();

    let sum = names.iter()
        .enumerate()
        .fold(0, |acc, (i, xs)| {
            let val = xs.iter().fold(0, |tot, x| tot + x);
            acc + val * (i + 1)
        });

    assert!(sum == 871198282);
    format!("p022 = {}", sum)
} // 871198282

/// Non-abundant sums
pub fn p023() -> String {
    const N: usize = 28124;
    let factor_sums = eu::divisor_sum_list(N - 1);
    let mut abundants = [false; N];
    for (i, item) in factor_sums.iter().enumerate() {
        abundants[i] = *item > i;
    }

    let mut sum = 0;
    for i in 0..N {
        for (j, abundant) in abundants.iter().enumerate().take(N) {
            if *abundant {
                if j >= i {
                    sum += i;
                    break;
                }
                if abundants[i - j] {
                    break;
                }
            }
        }
    }

    assert_eq!(sum, 4179871);
    format!("p023 = {}", sum)
} // 4179871

/// Lexicographic permutations
pub fn p024() -> String {
    let mut res: Vec<usize> = Vec::new();
    let mut n = 1_000_000;
    let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut e = a.len();

    for _ in 0..a.len() {
        let fact_e_str: &str = &*eu_big::factorial(e).to_string();
        let division = usize::from_str(fact_e_str).unwrap() / e;
        let pos = ((n as f64) / (division as f64)).ceil() as usize;
        res.push(a[pos - 1]);
        e -= 1;
        let t = a.clone();
        let left = &t[..pos - 1];
        let right = &t[pos..];
        a.clear();
        a.extend_from_slice(left);
        a.extend_from_slice(right);
        n -= division * (pos - 1);
    }

    let mut tt: String = String::new();
    for v in &res {
        tt.push_str(&*v.to_string())
    }

    assert_eq!(tt, "2783915460".to_string());
    format!("p024 = {}", tt)
} // 2783915460

/// 1000-digit Fibonacci number
pub fn p025() -> String {
    let bu10 = 10.to_biguint().unwrap();
    let mut limit: BigUint = One::one();
    for _ in 0..999 {
        limit = limit * &bu10
    }
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut cnt = 0;
    while a < limit {
        a = a + &b;
        mem::swap(&mut a, &mut b);
        cnt += 1;
    }

    assert_eq!(cnt, 4782);
    format!("p025 = {}", cnt)
} // 4782

/// Reciprocal cycles
pub fn p026() -> String {
    fn repeat_cnt(n: usize) -> usize {
        let mut cnt = 2;
        if n % 5 != 0 {
            let mut md = 10 % n;
            while md != 1 && cnt != n {
                md = (10 * md) % n;
                cnt += 1
            }
        }
        cnt
    }

    let (mut max, mut idx, mut i) = (0, 0, 3);
    while i < 1000 {
        let current = repeat_cnt(i);
        if current > max {
            max = current;
            idx = i
        }
        i += 2;
    }

    assert_eq!(idx, 983);
    format!("p026 = {}", idx)
} // 983

/// Quadratic primes
pub fn p027() -> String {
    fn eval_quad(a: i32, b: i32, sieve: &primal::Sieve) -> i32 {
        let mut cnt = 0;
        for i in 0.. {
            let v = i * i + a * i + b;
            if v > 0 && sieve.is_prime(v as usize) {
                cnt += 1
            } else {
                break;
            }
        }
        cnt
    }

    let sieve = primal::Sieve::new(15_000);
    let mut max = 0;
    let mut ab: (i32, i32) = (0, 0);
    for a in -1000..1001 {
        for b in 1..1001 {
            if sieve.is_prime(b) {
                let t = eval_quad(a, b as i32, &sieve);
                if t > max {
                    max = t;
                    ab = (a, b as i32);
                }
            }
        }
    }

    let res = ab.0 * ab.1;
    assert_eq!(res, -59231);
    format!("p027 = {}", res)
} // -59231

/// Number spiral diagonals
pub fn p028() -> String {
    let sum = (3..1002).step(2).fold(1, |acc, i| acc + 4 * i * i - 6 * (i - 1));
    assert_eq!(sum, 669171001);
    format!("p028 = {}", sum)
} // 669171001

/// Distinct powers
pub fn p029() -> String {
    let mut map = HashSet::new();
    for a in 2..101 {
        for b in 2..101 {
            map.insert(((b as f64) * (a as f64).ln()).to_string());
        }
    }

    assert_eq!(map.len(), 9240);
    format!("p029 = {}", map.len())
} // 9240

/// Digit fifth powers
pub fn p030() -> String {
    const B: u32 = 5;

    let sum_pow5 = |mut n: u32| -> u32 {
        let mut sum = 0;
        while n != 0 {
            sum += (n % 10).pow(B);
            n /= 10
        }
        sum
    };

    let max = (9 as u32).pow(B) * (B - 1);
    let res = (2..max).fold(0, |acc, x| if sum_pow5(x) == x { acc + x } else { acc });
    assert_eq!(res, 443839);
    format!("p030 = {}", res)
} // 443839

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (21, vec![p021, p022, p023, p024, p025, p026, p027, p028, p029, p030])
}
