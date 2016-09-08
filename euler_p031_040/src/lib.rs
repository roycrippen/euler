//! Project Euler solutions for problems 31 through 40.
//!
//! This crate is designed to be used via crate `euler`.

use std::collections::HashSet;
use std::f32::EPSILON;

extern crate primal;

extern crate num;
use num::integer::gcd;

extern crate euler_library;
use euler_library::common as eu;

/// Coin sums
pub fn p031() -> String {
    let mut ws = vec![0; 201];
    ws[0] = 1;
    let pences = vec![1, 2, 5, 10, 20, 50, 100, 200];
    for v in pences {
        for (j, _) in ws.clone().iter().enumerate().take(201).skip(v) {
            // for j in v..201 {
            ws[j] += ws[j - v]
        }
    }

    let res = *ws.last().unwrap();
    assert_eq!(res, 73682);
    format!("p031 = {}", res)
} // 73682

/// Pandigital products
pub fn p032() -> String {
    let mut m: HashSet<usize> = HashSet::new();
    // 1-digit * 4-digits in map to avoid dups
    for i in 2..10 {
        for j in 1000..10000 {
            let str = format!("{}{}{}", i, j, i * j);
            if str.len() != 9 {
                break; // don't continue to iterate if result is more than 9 digits
            }
            if eu::is_pandigital(str, 1) {
                m.insert(i * j);
            }
        }
    }

    // 2-digits * 3-digits in map to avoid dups
    for i in 11..100 {
        for j in 100..1000 {
            let str = format!("{}{}{}", i, j, i * j);
            if str.len() != 9 {
                break; // don't continue to iterate if result is more than 9 digits
            }
            if eu::is_pandigital(str, 1) {
                m.insert(i * j);
            }
        }
    }

    let sum = m.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 45228);
    format!("p032 = {}", sum)
} // 45228

/// Digit cancelling fractions
pub fn p033() -> String {
    fn is_digit_canceling_fraction(n: u32, d: u32) -> bool {
        let ns = format!("{}", n).into_bytes();
        let ds = format!("{}", d).into_bytes();
        if ns[0] == ds[1] {
            return ((ns[1] as f32 - 48.0) / (ds[0] as f32 - 48.0) - (n as f32) / (d as f32)).abs() < EPSILON;
        }
        if ns[1] == ds[0] {
            return ((ns[0] as f32 - 48.0) / (ds[1] as f32 - 48.0) - (n as f32) / (d as f32)).abs() < EPSILON;
        }
        false
    }

    // 2-digit / 2-digit combinations where numerator < denominator
    let (mut prod_num, mut prod_den) = (1, 1);
    for i in 10..100 {
        for j in i + 1..100 {
            if is_digit_canceling_fraction(i, j) {
                prod_num *= i;
                prod_den *= j
            }
        }
    }

    let answer = prod_den / gcd(prod_num, prod_den);
    assert_eq!(answer, 100);
    format!("p033 = {}", answer)
} // 100

/// Digit factorials
pub fn p034() -> String {
    let fact = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let is_digit_fact = |n: usize| -> bool {
        let (mut val, mut t) = (0, n);
        while t != 0 {
            val += fact[t % 10];
            t /= 10
        }
        val == n
    };

    // max value 5*9! = 1814400 < 10^7
    let sum = (10..1814401).fold(0, |acc, x| if is_digit_fact(x) { acc + x } else { acc });
    assert_eq!(sum, 40730);
    format!("p034 = {}", sum)
} // 40730

/// Circular primes
pub fn p035() -> String {
    fn rotate(mut vec: Vec<u8>) -> Vec<u8> {
        let first = vec.remove(0);
        vec.push(first);
        vec
    }

    fn next_candidate(n: usize) -> usize {
        eu::from_bytes(&rotate(eu::to_bytes(n))).unwrap()
    }

    let sieve = primal::Sieve::new(1_000_000);

    let is_circular_prime = |n: usize| -> bool {
        if sieve.is_prime(n) {
            // can not contain 0,2,4,6,8
            let s = n.to_string();
            if s.find('0') != None || s.find('2') != None || s.find('4') != None || s.find('6') != None ||
               s.find('8') != None {
                return false;
            }
            let mut next = next_candidate(n);
            while next != n {
                if !sieve.is_prime(next) {
                    return false;
                }
                next = next_candidate(next);
            }
            return true;
        }
        false
    };

    let mut sum = 0;
    let mut i = 1;
    while i < 1_000_000 {
        if is_circular_prime(i) {
            sum += 1
        }
        i += 2;
    }

    assert_eq!(sum + 1, 55);
    format!("p035 = {}", sum + 1)
} // 55

/// Double-base palindromes
pub fn p036() -> String {
    fn is_palindrome_base10(n: usize) -> bool {
        let mut rev = 0;
        let mut num = n;
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10
        }
        rev == n
    }

    fn is_palindrome_binary(n: usize) -> usize {
        let s = format!("{:b}", n);
        let fwd = s.chars().clone();
        let rev = s.chars().rev();
        if fwd.eq(rev) {
            return n;
        }
        0
    }

    let a = (1..1000000).fold(0,
                              |acc, x| if is_palindrome_base10(x) { acc + is_palindrome_binary(x) } else { acc });
    assert_eq!(a, 872187);
    format!("p036 = {}", a)
} // 872187

/// Truncatable primes
pub fn p037() -> String {
    fn trunc_left(n: usize) -> usize {
        let mut xs = eu::to_bytes(n);
        if xs.len() < 2 {
            return n;
        }
        xs.remove(0);
        eu::from_bytes(&xs).unwrap()
    }

    let sieve = primal::Sieve::new(1_000_000);

    let is_trunc_prime = |n: usize| -> bool {
        // eliminate 2,3,5,7
        if n / 10 == 0 {
            return false;
        }
        let mut num = n;
        // check trunc right first - faster
        while num != 0 {
            if !sieve.is_prime(num) {
                return false;
            }
            num /= 10;
        }
        num = n;
        // check trunc left
        loop {
            if !sieve.is_prime(num) {
                return false;
            }
            num = trunc_left(num);
            if num / 10 == 0 {
                if sieve.is_prime(num) {
                    break;
                } else {
                    return false;
                }
            }
        }
        true
    };


    let mut trunc_primes: Vec<usize> = Vec::new();
    for i in 11..1_000_000 {
        if is_trunc_prime(i) {
            trunc_primes.push(i)
        }
        if trunc_primes.len() == 11 {
            break;
        }
    }

    let sum = trunc_primes.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 748317);
    format!("p037 = {}", sum)
} // 748317

/// Pandigital multiples
pub fn p038() -> String {
    // 9 * (1,2,3,4,5)
    let mut max = String::from("918273645");

    // try 2 digit multiplier
    for i in 91..100 {
        let s = format!("{}{}{}", i, 2 * i, 3 * i);
        if s.len() != 9 && eu::is_pandigital(s.clone(), 1) && s > max {
            max = s
        }
    }
    // try 4 digit multiplier
    for i in 9123..9899 {
        let s = format!("{}{}", i, 2 * i);
        if s.len() != 9 {
            break;
        }
        if eu::is_pandigital(s.clone(), 1) && s > max {
            max = s
        }
    }

    assert_eq!(max, "932718654".to_string());
    format!("p038 = {}", max)
} // 932718654

/// Integer right triangles
pub fn p039() -> String {
    let mut ps = vec![0; 1001];

    for a in 3..999 {
        for b in (a + 1)..999 {
            let hypot = ((a * a + b * b) as f64).sqrt();
            if hypot - hypot.floor() == 0.0 {
                let p = a + b + (hypot as usize);
                if p > 1000 {
                    break;
                }
                ps[p] += 1
            }
        }
    }

    let max = ps.iter().max().unwrap();
    let mut res = 0;
    for (i, p) in ps.iter().enumerate().skip(3) {
        // for i in 3..ps.len() {
        if *p == *max {
            res = i;
            break;
        }

    }

    assert_eq!(res, 840);
    format!("p039 = {}", res)
} // 840

/// Champernowne's constant
pub fn p040() -> String {
    let (mut prod, mut cnt, mut next) = (1, 1, 1);
    for i in 1.. {
        let ds = eu::to_bytes(i);
        if next >= cnt && next < (cnt + ds.len()) {
            prod *= ds[next - cnt] - 48;
            next *= 10;
            if next > 1_000_000 {
                break;
            }
        }
        cnt += ds.len();
    }

    assert_eq!(prod, 210);
    format!("p040 = {}", prod)
} // 210

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (31, vec![p031, p032, p033, p034, p035, p036, p037, p038, p039, p040])
}
