//! Project Euler solutions for problems 1 through 10.

use std::ops::Add;
use std::f64::EPSILON;

extern crate primal;

extern crate itertools;
use self::itertools::Itertools;

extern crate euler_library;
use self::euler_library::common as eu;

/// Multiples of 3 and 5
pub fn eu001() -> String {
    let n = 1000;
    let res = (0..n).fold(0,
                          |acc, x| if x % 3 == 0 || x % 5 == 0 { acc + x } else { acc });

    assert_eq!(res, 233168);
    format!("eu001 = {}", res)
} // 233168

/// Even Fibonacci numbers
pub fn eu002() -> String {
    struct Fibonacci {
        curr: usize,
        next: usize,
    }

    impl Iterator for Fibonacci {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    // Returns a fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }

    fn solve(n: usize) -> usize {
        fibonacci()
            .take_while(|&x| x < n)
            .filter(|x| x % 2 == 0)
            .scan(0, |acc, fib| {
                *acc = *acc + fib;
                Some(*acc)
            })
            .max()
            .unwrap()
    }
    assert_eq!(solve(50), 44);

    let sum = solve(4_000_000);
    assert_eq!(sum, 4613732);
    format!("eu002 = {}", sum)
} // 4613732


/// Largest prime factor
pub fn eu003() -> String {
    let sieve = primal::Sieve::new(10_000);
    let (res, _) = sieve.factor(600851475143).unwrap().into_iter().max().unwrap();
    assert_eq!(res, 6857);
    format!("eu003 = {}", res)
} // 6857

/// Largest palindrome product
pub fn eu004() -> String {
    fn solve() -> usize {
        let mut max = 0;
        let it = (99..999).rev();
        for i in it.clone() {
            for j in it.clone() {
                let t = i * j;
                if t > max && eu::is_palindrome(t) {
                    max = t
                }
                if t < max {
                    break;
                }
            }
        }
        max
    }

    let max = solve();
    assert_eq!(max, 906609);
    format!("eu004 = {}", max)
} // 906609

/// Smallest multiple
pub fn eu005() -> String {
    fn solve() -> usize {
        for i in (2520..).step(2520) {
            if i % 19 + i % 18 + i % 17 + i % 16 + i % 15 + i % 14 + i % 13 + i % 12 + i % 11 == 0 {
                return i;
            }
        }
        0
    }

    let res = solve();
    assert_eq!(res, 232792560);
    format!("eu005 = {}", res)
} // 232792560

/// Sum square difference
pub fn eu006() -> String {
    fn solve(n: usize) -> usize {
        let sum = (1..n + 1).fold(0, Add::add);
        let sum_square = (1..n + 1).fold(0, |acc, x| acc + x * x);
        sum * sum - sum_square
    }
    assert_eq!(solve(10), 2640);

    let res = solve(100);
    assert_eq!(res, 25164150);
    format!("eu006 = {}", res)
} // 25164150

/// 10001st prime
pub fn eu007() -> String {
    let nth_prime = primal::Primes::all().nth(10001 - 1).unwrap();
    assert_eq!(nth_prime, 104743);
    format!("eu007 = {}", nth_prime)
} // 104743

/// Largest product in a series
pub fn eu008() -> String {
    let vals = include_str!("../data/p008_product.txt")
                   .bytes()
                   .filter(|&b| b != 10)
                   .map(|x| x as usize - 48)
                   .collect::<Vec<_>>();

    let max = (0..(vals.len() - 12))
                  .map(|i| {
                      vals.iter()
                          .take(i + 13)
                          .skip(i)
                          .fold(1, |acc, x| acc * x)
                  })
                  .max()
                  .unwrap();

    assert_eq!(max, 23514624000);
    format!("eu008 = {}", max)
} // 23514624000

/// Special Pythagorean triplet
pub fn eu009() -> String {
    let res = (1..500)
                  .flat_map(|a| {
                      (a..500).filter_map(move |b| {
                          let c = ((a * a + b * b) as f64).sqrt();
                          let circum = a + b + (c as usize);
                          if c.fract() < EPSILON && circum == 1000 { Some(a * b * (c as usize)) } else { None }
                      })
                  })
                  .nth(0)
                  .unwrap();

    assert_eq!(res, 31875000);
    format!("eu009 = {}", res)
} // 31875000

/// Summation of primes
pub fn eu010() -> String {
    fn solve(n: usize) -> usize {
        match n {
            0...1 => 0,
            2 => 2,
            3...4 => 5,
            5...7 => 10,
            _ => {
                let sieve = primal::Sieve::new(2_000_000);
                (7..n)
                    .step(2)
                    .fold(0,
                          |acc, i| if i % 5 != 0 && sieve.is_prime(i) { acc + i } else { acc }) + 10
            }
        }
    }
    assert_eq!(solve(9), 17);

    let sum = solve(2_000_000);
    assert_eq!(sum, 142913828922);
    format!("eu010 = {}", sum)
} // 142913828922

/// Returns Vec of the Euler solution functions in this crate.
pub fn get_functions() -> Vec<fn() -> String> {
    // Euler solutions in this crate.
    vec![eu001, eu002, eu003, eu004, eu005, eu006, eu007, eu008, eu009, eu010]
}
