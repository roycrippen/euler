//! Project Euler solutions for problems 11 through 20.

use std::str::FromStr;
use std::cmp;

extern crate itertools;
use self::itertools::Itertools;

extern crate num;
use self::num::{BigUint, pow};
use self::num::bigint::ToBigUint;

extern crate euler_library;
use self::euler_library::big as eu_big;


/// Largest product in a grid
pub fn eu011() -> String {
    fn get_data() -> Vec<Vec<usize>> {
        let buffer = include_str!("../data/p011_grid.txt");
        buffer.lines()
              .map(|x| {
                  x.split(',')
                   .map(|x| x.parse().unwrap())
                   .collect::<Vec<usize>>()
              })
              .collect::<Vec<Vec<usize>>>()
    }

    let data = get_data();
    let mut max = 0;

    for (i, vi) in data.iter().enumerate().take(20) {
        for (j, _) in vi.iter().enumerate().take(20) {

            let mut t = data[i][j] * data[i][j + 1] * data[i][j + 2] * data[i][j + 3];
            if t > max {
                max = t
            }
            // down
            t = data[i][j] * data[i + 1][j] * data[i + 2][j] * data[i + 3][j];
            if t > max {
                max = t
            }
            // diag right
            t = data[i][j] * data[i + 1][j + 1] * data[i + 2][j + 2] * data[i + 3][j + 3];
            if t > max {
                max = t
            }
            // diag left
            t = data[i][j + 3] * data[i + 1][j + 2] * data[i + 2][j + 1] * data[i + 3][j];
            if t > max {
                max = t
            }
        }
    }

    assert_eq!(max, 70600674);
    format!("eu011 = {}", max)
} // 70600674


/// Highly divisible triangular number
pub fn eu012() -> String {
    // Returns cnt of factors of n excluding 1 and n
    fn factor_cnt(mut n: usize) -> usize {
        if n < 2 {
            return 1;
        }
        let mut factors = 1;
        let max = (n as f64).sqrt() as usize;
        for i in 2.. {
            if i > max {
                break;
            }
            let mut p = 0;
            while n % i == 0 {
                p += 1;
                n /= i
            }
            factors *= p + 1;
        }
        factors
    }
    assert_eq!(factor_cnt(21), 2);
    assert_eq!(factor_cnt(28), 3);

    fn solve() -> usize {
        for i in (2..).step(4) {
            let temp = factor_cnt(i + 1);
            if temp * factor_cnt(i / 2) > 500 {
                return i * (i + 1) / 2;
            }
            if temp * factor_cnt((i + 2) / 2) > 500 {
                return (i + 1) * (i + 2) / 2;
            }
        }
        0
    }

    let tri_number = solve();
    assert_eq!(tri_number, 76576500);
    format!("eu012 = {}", tri_number)
} // 76576500

/// Large sum
pub fn eu013() -> String {
    let buffer = include_str!("../data/p013_sum.txt");
    let xs: Vec<&str> = buffer.split_whitespace().collect();
    let sum = xs.into_iter()
                .fold(0.to_biguint().unwrap(), |acc, x| {
                    let bu: BigUint = FromStr::from_str(x).unwrap();
                    acc + bu
                });

    let str = &sum.to_string()[..10];
    assert_eq!(str, "5537376230");
    format!("eu013 = {}", str)
} // 5537376230

/// Longest Collatz sequence
pub fn eu014() -> String {
    const LIMIT: usize = 1_000_000;
    let mut cache: Vec<usize> = vec![0; LIMIT];

    let mut max = 0;
    let mut answer = 1;
    for (i, _) in cache.clone().iter().enumerate().skip(1) {
        let mut n = i;
        let mut cnt = 0;
        while n != 1 {
            cnt += 1;
            n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
            if n < i {
                cnt += cache[n];
                break;
            }
        }
        cache[i] = cnt;
        if cnt > max {
            max = cnt;
            answer = i
        }
    }

    assert_eq!(answer, 837799);
    format!("eu014 = {}", answer)
} // 837799

/// Lattice paths
pub fn eu015() -> String {
    // C(n,r) = n! / ( r! (n - r)! )
    // 40! / (20! (40 - 20)!)
    let fact_n = eu_big::factorial(40);
    let fact_r = eu_big::factorial(20);
    let s = (&fact_n / (&fact_r * &fact_r)).to_string();

    assert_eq!(s, "137846528820".to_string());
    format!("eu015 = {}", s)
} // 137846528820

/// Power digit sum
pub fn eu016() -> String {
    let n = 1000;
    let two = 2.to_biguint().unwrap();
    let xs = pow(two, n).to_string();
    let res = xs.chars().fold(0, |acc, x| acc + (x as usize) - 48);

    assert_eq!(res, 1366);
    format!("eu016 = {}", res)
} // 1366

/// Maximum path sum I
pub fn eu017() -> String {
    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }
    let m = hashmap![1 => "one", 2 => "two", 3 => "three", 4 => "four", 5 => "five", 6 => "six", 7 => "seven",
		8 => "eight", 9 => "nine", 10 => "ten", 11 => "eleven", 12 => "twelve", 13 => "thirteen", 14 => "fourteen",
		15 => "fifteen", 16 => "sixteen", 17 => "seventeen", 18 => "eighteen", 19 => "nineteen", 20 => "twenty",
		30 => "thirty", 40 => "forty", 50 => "fifty", 60 => "sixty", 70 => "seventy", 80 => "eighty", 90 => "ninety"];

    let m_len = |n: usize| -> usize { m.get(&n).unwrap().len() };

    let one_to_nine = (1..10).fold(0, |acc, x| acc + m_len(x));

    let one_to_nineteen = &one_to_nine + (10..20).fold(0, |acc, x| acc + m_len(x));

    let tys = m_len(20) + m_len(30) + m_len(40) + m_len(50) + m_len(60) + m_len(70) + m_len(80) + m_len(90);

    let one_to_ninety_nine = &one_to_nineteen + 8 * &one_to_nine + 10 * &tys;

    let res = 100 * &one_to_nine + 9 * ("hundred".len() + 99 * "hundredand".len()) + 10 * &one_to_ninety_nine +
              "onethousand".len();

    assert_eq!(res, 21124);
    format!("eu017 = {}", res)
} // 21124

/// Maximum path sum I
pub fn eu018() -> String {
    fn get_data() -> Vec<Vec<usize>> {
        let buffer = include_str!("../data/p018_triangle.txt");
        buffer.lines()
              .map(|x| {
                  x.split(' ')
                   .map(|x| x.parse().unwrap())
                   .collect::<Vec<usize>>()
              })
              .collect::<Vec<Vec<usize>>>()
    }

    let mut xss = get_data();
    for i in (0..xss.len()).rev() {
        for j in 0..i {
            xss[i - 1][j] += cmp::max(xss[i][j], xss[i][j + 1])
        }
    }

    assert_eq!(xss[0][0], 1074);
    format!("eu018 = {}", xss[0][0])
} // 1074

/// Counting Sundays
pub fn eu019() -> String {
    let mut sunday = 7;
    let mut cnt = 0;
    for i in 1900..2001 {
        let days = if i % 4 == 0 && (i % 100 != 0 || i % 400 == 0) { 366 } else { 365 };
        while sunday < days {
            if i != 1900 {
                if days == 365 {
                    match sunday {
                        1 | 32 | 60 | 91 | 121 | 152 | 182 | 213 | 244 | 274 | 305 | 335 => cnt += 1,
                        _ => (),
                    }
                } else {
                    match sunday {
                        1 | 32 | 61 | 92 | 122 | 153 | 183 | 214 | 245 | 275 | 306 | 336 => cnt += 1,
                        _ => (),
                    }
                }
            }
            sunday += 7;
        }
        sunday = if sunday == 0 { 7 } else { sunday - days }
    }

    assert_eq!(cnt, 171);
    format!("eu019 = {}", cnt)
} // 171

/// Factorial digit sum
pub fn eu020() -> String {
    let n = 100;
    let xs = eu_big::factorial(n).to_string();
    let res = xs.chars().fold(0, |acc, x| acc + (x as usize) - 48);

    assert_eq!(res, 648);
    format!("eu020 = {}", res)
} // 648

/// Returns Vec of the Euler solution functions in this crate.
pub fn get_functions() -> Vec<fn() -> String> {
    vec![eu011, eu012, eu013, eu014, eu015, eu016, eu017, eu018, eu019, eu020]
}
