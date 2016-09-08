//! Project Euler solutions for problems 121 through 130.
//!
//! This crate is designed to be used via crate `euler`.

extern crate euler_library;
extern crate num;
extern crate primal;

use euler_library::common as eu;
use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;

/// Disc game prize fund
pub fn p121() -> String {
    // hints from https://github.com/juanplopes/euler/blob/master/121.boo
    // calculate p for a blue/red winning set (ie more bluea than red)
    // for n = 4, p([1,0,1,1] = 1/2 * 2/3 * 1/4 * 1/5)
    fn p_win(xs: &[usize]) -> f64 {
        xs.iter()
            .enumerate()
            .fold(1.0, |acc, (i, &x)| {
                if x != 0 { acc * (1.0 / (2.0 + i as f64)) } else { acc * (1.0 + i as f64) / (2.0 + i as f64) }
            })
    }

    fn solve(n: u32) -> usize {
        let max = (2 as usize).pow(n);
        let blues_needed = n / 2 + 1;
        // create permutations iterator of winning outcomes, for n = 4 ws is
        // ws.collect() = [[0, 1, 1, 1], [1, 0, 1, 1], [1, 1, 0, 1], [1, 1, 1, 0], [1, 1, 1, 1]]
        let ws = (0..max).filter_map(|i| {
            let s = format!("{:015b}", i)
                .into_bytes()
                .iter()
                .map(|y| (y - 48) as usize)
                .collect::<Vec<_>>();
            let enough_blues = s.iter().fold(0, |acc, v| acc + *v as u32) >= blues_needed;
            if enough_blues { Some(s) } else { None }
        });
        // send just subslice  of xs to p_win() for testing below n = 15.
        // format! takes literals only so ws elements have length of 15
        (1.0 / ws.fold(0.0, |acc, xs| acc + p_win(&xs[(15 - n as usize)..]))) as usize
    }

    assert_eq!(solve(4), 10);

    let res = solve(15);
    assert_eq!(res, 2269);
    format!("p121 = {}", res)
} // 2269

/// Efficient exponentiation
pub fn p122() -> String {
    fn path(n: usize, p: &mut HashMap<usize, usize>, lvl: &mut Vec<usize>) -> Vec<usize> {
        match n {
            0 => return vec![],
            1 => return vec![1],
            2 => return vec![1, 2],
            3 => return vec![1, 2, 3],
            4 => return vec![1, 2, 4],
            5 => return vec![1, 2, 3, 5],
            6 => return vec![1, 2, 3, 6],
            7 => return vec![1, 2, 3, 5, 7],
            8 => return vec![1, 2, 4, 8],
            9 => return vec![1, 2, 4, 5, 9],
            10 => return vec![1, 2, 3, 5, 10],
            11 => return vec![1, 2, 3, 5, 10, 11],
            12 => return vec![1, 2, 3, 6, 12],
            _ => {
                while !p.contains_key(&n) {
                    let mut q = lvl.clone();
                    lvl.clear();
                    for x in q.clone() {
                        for y in path(x, p, &mut q) {
                            if x + y > 2 * n || p.contains_key(&(x + y)) {
                                continue;
                            }
                            p.insert(x + y, x);
                            lvl.push(x + y);
                        }
                    }
                }

                let mut res = path(*p.get(&n).unwrap(), p, lvl);
                res.push(n);
                res
            }
        }
    }

    fn m(n: usize) -> usize {
        let mut p = &mut HashMap::new();
        p.insert(1, 0);
        let xs = path(n, p, &mut vec![1]);
        // println!("M({:3}) = {:2}, {:?}", n, xs.len() - 1, xs);
        xs.len() - 1
    }

    fn solve(n: usize) -> usize {
        let mut sum = 0;
        for i in 1..n + 1 {
            sum += m(i);
        }
        sum
    }

    let res = solve(200);
    assert_eq!(res, 1582);
    format!("p122 = {}", res)
} // 1582


/// Prime square remainders
pub fn p123() -> String {
    fn solve() -> usize {
        let max = (10 as usize).pow(10);
        let sieve = primal::Sieve::new(250_000);
        (7037..).take_while(|i| 2 as usize * i * sieve.nth_prime(i - 1) < max).max().unwrap() + 1
    }

    let res = solve();
    assert_eq!(res, 21035);
    format!("p123 = {}", res)
} // 21035

/// Ordered radicals
pub fn p124() -> String {
    const MAX: usize = 100_001;

    fn get_rads() -> Vec<(usize, usize)> {
        let sieve = primal::Sieve::new(MAX);
        let mut rads = [1 as usize; MAX];
        for n in 1..sieve.prime_pi(MAX) + 1 {
            let p = sieve.nth_prime(n);
            let mut i = p;
            while i < MAX {
                rads[i] *= p;
                i += p;
            }
        }
        let mut rads = rads.iter().enumerate().map(|(i, &x)| (x, i)).collect::<Vec<_>>();
        rads.sort();
        rads
    }

    let (_, res) = get_rads()[10_000];
    assert_eq!(res, 21417);
    format!("p124 = {}", res)
} // 21417


/// Palindromic sums
pub fn p125() -> String {
    fn sof_sqrs(n: usize) -> Vec<usize> {
        let limit = (n as f64).sqrt() as usize + 1;
        (0..limit)
            .scan(0, |state, x| {
                *state = *state + x * x;
                Some(*state)
            })
            .collect()
    }

    fn palindromic_sof_sqrs(n: usize) -> HashSet<usize> {
        let sofs = sof_sqrs(n);
        let mut res = HashSet::new();
        for (i, vi) in sofs.iter().enumerate().take(sofs.len() - 2) {
            for vj in sofs.iter().skip(i + 2) {
                let v = vj - vi;
                if v >= n {
                    break;
                }
                if eu::is_palindrome(v) {
                    res.insert(v);
                }
            }
        }
        res
    }

    let res = palindromic_sof_sqrs(1000).iter().fold(0, |acc, x| acc + x);
    assert_eq!(res, 4164);

    let res = palindromic_sof_sqrs((10 as usize).pow(8)).iter().fold(0, |acc, x| acc + x);
    assert_eq!(res, 2906969179);
    format!("p125 = {}", res)
} // 2906969179

/// Cuboid layers
pub fn p126() -> String {
    fn f(x: u32, y: u32, z: u32, l: u32) -> u32 {
        2 * (x * y + x * z + y * z) + 4 * (l - 1) * (x + y + z + l - 2)
    }

    fn solve(n: u32) -> usize {
        let limit = n * 20;
        let mut xs = vec![0;limit as usize+1];

        for x in 1..limit {
            if f(x, 1, 1, 1) > limit {
                break;
            }
            for y in 1..(x + 1) {
                if f(x, y, 1, 1) > limit {
                    break;
                }
                for z in 1..(y + 1) {
                    for l in 1.. {
                        let cnt = f(x, y, z, l);
                        if cnt > limit {
                            break;
                        }
                        xs[cnt as usize] += 1
                    }
                }
            }
        }
        xs.iter().position(|&x| x == n).unwrap()
    }

    let res = solve(1000);
    assert_eq!(res, 18522);
    format!("p126 = {}", res)
} // 18522

/// abc-hits - unimplemented
pub fn p127() -> String {

    const MAX: usize = 120001;

    #[derive(Debug, Clone, Copy)]
    struct Rad {
        n: usize,
        rad: usize,
    }

    fn get_rads() -> Vec<Rad> {
        let sieve = primal::Sieve::new(MAX);
        let mut rads = vec![Rad { n: 1, rad: 1 }; MAX];

        for n in 1..sieve.prime_pi(MAX) + 1 {
            let p = sieve.nth_prime(n);
            let mut i = p;
            while i < MAX {
                rads[i].rad *= p;
                i += p;
            }
        }

        for (i, v) in rads.iter_mut().enumerate() {
            v.n = i
        }

        rads
    }

    let rads = get_rads();
    let mut rads_sorted = rads.clone();
    rads_sorted.sort_by(|a, b| a.rad.cmp(&b.rad));
    let mut res = 0;

    for c in rads.iter().skip(3) {
        let c_div_2 = c.n / 2;
        for a in rads_sorted.iter().skip(1) {
            if a.n >= c_div_2 {
                continue;
            }
            if a.rad * c.rad > c_div_2 {
                break;
            };
            let b = c.n - a.n;
            if a.rad * rads[b].rad * c.rad < c.n && gcd(a.rad, rads[b].rad) == 1 {
                res += c.n
            }
        }
    }

    assert_eq!(res, 18407904);
    format!("p127 = {}", res)
} // 18407904

/// Hexagonal tile differences - unimplemented
pub fn p128() -> String {
    "p128 = unimplemented".to_string()
}

/// Repunit divisibility - unimplemented
pub fn p129() -> String {
    "p129 = unimplemented".to_string()
}

/// Composites with prime repunit property - unimplemented
pub fn p130() -> String {
    "p130 = unimplemented".to_string()
}

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    (121, vec![p121, p122, p123, p124, p125, p126, p127])
}
