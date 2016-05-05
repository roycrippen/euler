//! Project Euler solutions for problems 41 through 50.
//!
//! This crate is designed to be used via crate `euler`.

use std::str::FromStr;
use std::f64::EPSILON;

extern crate primal;

extern crate permutohedron;
use permutohedron::Heap;

extern crate euler_library;
use euler_library::common as eu;
use euler_library::primes;

/// Pandigital prime
pub fn p041() -> String {
    const LIMIT: usize = 10_000_000;
    let sieve = primal::Sieve::new(LIMIT);

    // pandigital 8 and 9 digits long are dividible by 3 and can not be prime
    let mut i = LIMIT - 1;
    while i > 2 {
        if sieve.is_prime(i) && eu::is_pandigital(i.to_string(), 1) {
            break;
        }
        i -= 2;
    }

    assert_eq!(i, 7652413);
    format!("p041 = {}", i)
} // 7652413

/// Coded triangle numbers
pub fn p042() -> String {
    fn get_data() -> Vec<Vec<u8>> {
        let buffer = include_str!("../data/p042_words.txt")
                         .chars()
                         .filter(|&x| x != '\"' && x != '\n' && x != '\"')
                         .collect::<String>();

        buffer.split(',')
              .map(|x| eu::to_bytes(x).into_iter().map(|y| y - 64).collect::<Vec<_>>())
              .collect::<Vec<_>>()
    }

    fn is_triangle_num(n: f64) -> bool {
        // using quadractic formula
        let quad = ((8.0 * n + 1.0).sqrt() - 1.0) / 2.0;
        (quad.floor() - quad).abs() < EPSILON
    }

    let names = get_data();
    let mut cnt = 0;
    for name in names {
        let sum = name.iter().fold(0, |acc, x| acc + x);
        if is_triangle_num(sum as f64) {
            // println!("{:?}", sum);
            cnt += 1
        }
    }

    assert_eq!(cnt, 162);
    format!("p042 = {}", cnt)
} // 162

/// Sub-string divisibility
pub fn p043() -> String {

    // 4 hefty helper functions.  very fast solution - .006 sec
    fn get_next_grp(xss: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = Vec::new();
        for xs in xss {
            let chs = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            for c in chs {
                let xs = &mut xs.clone();
                xs.pop();
                let mut t: Vec<char> = Vec::new();
                t.push(c);
                t.append(xs);
                let mut tt = t.clone();
                tt.sort();
                tt.dedup();
                if tt.len() == 3 {
                    res.push(t)
                }
            }
        }
        res.sort();
        res.dedup();
        res
    }

    fn reduce_grp(xss: Vec<Vec<char>>, m: usize) -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = Vec::new();
        for xs in xss {
            let mut s = "".to_string();
            for x in xs.clone() {
                s.push(x)
            }
            let n: usize = usize::from_str(&s).unwrap();
            if n % m == 0 {
                res.push(xs);
            }
        }
        res
    }

    fn merge(xss: Vec<Vec<char>>, yss: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = Vec::new();
        for xs in xss {
            for ys in yss.clone() {
                if xs[xs.len() - 2] == ys[0] && xs[xs.len() - 1] == ys[1] {
                    let mut t: Vec<char> = Vec::new();
                    for x in xs.iter().take(xs.len() - 2) {
                        // for i in 0..xs.len() - 2 {
                        t.push(*x)
                    }
                    let mut ys = ys.clone();
                    t.append(&mut ys);
                    res.push(t)
                }
            }
        }
        res
    }

    fn final_reduce(xss: Vec<Vec<char>>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        for xs in xss {
            let mut s = "".to_string();
            for x in xs.clone() {
                s.push(x)
            }
            let n: usize = usize::from_str(&s).unwrap();
            if eu::is_pandigital(s, 0) {
                res.push(n);
            }
        }
        res
    }

    // start of solution
    let chs = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut dg_seventeen = eu::perms_without_reps_recur(3, &chs.clone());
    dg_seventeen = reduce_grp(dg_seventeen, 17);

    let mut dg_thirteen = get_next_grp(dg_seventeen.clone());
    dg_thirteen = reduce_grp(dg_thirteen, 13);

    let mut dg_eleven = get_next_grp(dg_thirteen.clone());
    dg_eleven = reduce_grp(dg_eleven, 11);

    let mut dg_seven = get_next_grp(dg_eleven.clone());
    dg_seven = reduce_grp(dg_seven, 7);

    let mut dg_five = get_next_grp(dg_seven.clone());
    dg_five = reduce_grp(dg_five, 5);

    let mut dg_three = get_next_grp(dg_five.clone());
    dg_three = reduce_grp(dg_three, 3);

    let mut dg_two = get_next_grp(dg_three.clone());
    dg_two = reduce_grp(dg_two, 2);

    let mut ms = merge(dg_two, dg_three);
    ms = merge(ms, dg_five);
    ms = merge(ms, dg_seven);
    ms = merge(ms, dg_eleven);
    ms = merge(ms, dg_thirteen);
    ms = merge(ms, dg_seventeen);

    let mut res: Vec<Vec<char>> = Vec::new();
    for v in ms {
        for c in chs.clone() {
            let mut t: Vec<char> = Vec::new();
            t.push(c);
            t.append(&mut v.clone());
            res.push(t)
        }
    }

    let list = final_reduce(res);
    let sum = list.iter().fold(0, |acc, x| acc + x);

    assert_eq!(sum, 16695334890);
    format!("p043 = {}", sum)
} // 16695334890

/// Pentagon numbers
pub fn p044() -> String {
    fn is_pent(x: usize) -> bool {
        let n = (1.0 + (1.0 + 24.0 * x as f64).sqrt()) / 6.0;
        n - n.floor() < EPSILON
    }

    fn solve() -> usize {
        // first value hit will be the smallest
        for (idx, j) in (1..).map(|x| (x, x * (3 * x - 1) / 2)) {
            for k in (1..idx).rev().map(|x| x * (3 * x - 1) / 2) {
                if is_pent(j - k) && is_pent(j + k) {
                    return j - k;
                }
            }
        }
        0
    }

    let res = solve();
    assert_eq!(res, 5482660);
    format!("p044 = {}", res)
} // 5482660

/// Triangular, pentagonal, and hexagonal
pub fn p045() -> String {
    // ignore triangles, all hexagonals are triangles
    fn solve() -> usize {

        let mut pent_stream = (166..).map(|x| x * (3 * x - 1) / 2).filter(|x| x % 5 == 0);
        let mut hex_stream = (144..).map(|x| x * (2 * x - 1)).filter(|x| x % 5 == 0);

        let mut pent = 0;
        loop {
            let hex = hex_stream.next().unwrap();
            while pent <= hex {
                pent = pent_stream.next().unwrap();
                if pent == hex {
                    return pent;
                }
            }
        }

    }

    let pent = solve();
    assert_eq!(pent, 1533776805);
    format!("p045 = {}", pent)
} // 1533776805

/// Goldbach's other conjecture
pub fn p046() -> String {
    let sieve = primal::Sieve::new(10_000);
    let mut i = 9;
    loop {
        if !sieve.is_prime(i) {
            let mut found = true;
            let end = ((i as f64) / 2.0).sqrt() as usize;
            for j in 1..end + 1 {
                if sieve.is_prime(i - 2 * j * j) {
                    found = false;
                    break;
                }
            }
            if found {
                break;
            }
        }
        i += 2;
    }

    assert_eq!(i, 5777);
    format!("p046 = {}", i)
} // 5777

/// Distinct primes factors
pub fn p047() -> String {
    fn four_distinct() -> usize {
        let pfcs = primes::prime_factor_cnt(200_000);
        let pfc = |n: usize| pfcs[n];

        for i in 646..200_000 {
            if pfc(i) == 4 && (1..4).all(|j| pfc(i + j) == 4) {
                return i;
            }
        }
        0
    }

    let res = four_distinct();
    assert_eq!(res, 134043);
    format!("p047 = {}", res)
} // 134043

/// Self powers
pub fn p048() -> String {
    let digs = 10_000_000_000;
    let mut res: usize = 0;
    for i in 1..1_000 {
        let mut term = 1;
        for _ in 1..i + 1 {
            term = (term * i) % digs;
        }
        res += term;
    }

    assert_eq!(res % digs, 9110846700);
    format!("p048 = {:?}", res % digs)
} // 9110846700

/// Prime permutations
pub fn p049() -> String {
    fn find_delta(xs: Vec<i32>, pos: usize) -> Option<i32> {
        let mut delta;
        for i in pos..xs.len() - 1 {
            delta = xs[i + 1];
            for x in xs.iter().skip(i + 2) {
                // for j in i + 2..xs.len() {
                if *x == delta * 2 {
                    return Some(delta);
                }
            }
        }
        None
    }

    let sieve = primal::Sieve::new(10_000);

    let get_seq = |n: usize| -> Option<(usize, i32)> {
        let mut vec = eu::to_bytes(n);
        let mut heap = Heap::new(&mut vec);
        let perms = heap.by_ref().collect::<Vec<_>>();
        let mut xs = perms.iter().map(|x| eu::from_bytes(&x.clone()).unwrap()).collect::<Vec<_>>();
        xs.sort();
        xs.dedup();
        let ys = xs.iter().filter(|x| sieve.is_prime(**x) && **x > 1000).collect::<Vec<_>>();
        if ys.len() < 4 {
            return None;
        }
        for (i, v1) in ys.iter().enumerate().take(ys.len() - 2) {
            let mut diffs = Vec::new();
            for v2 in ys.clone() {
                diffs.push(((**v1 as i32) - (*v2 as i32)).abs());
            }
            let delta = find_delta(diffs, i);
            if delta != None {
                return Some((*ys[i], delta.unwrap()));
            }
        }
        None
    };

    let mut res: String = "".to_string();
    for i in 1488..10_000 {
        if !sieve.is_prime(i) {
            continue;
        }
        if let Some(tup) = get_seq(i) {
            let (v, diff) = tup;
            if v < i {
                continue;
            }
            res = format!("{}{}{}", v, v + diff as usize, v + 2 * diff as usize);
            break;
        }
    }

    assert_eq!(res, "296962999629".to_string());
    format!("p049 = {}", res)
} // 296962999629

/// Consecutive prime sum
pub fn p050() -> String {
    const MAX: usize = 1_000_001;

    let sieve = primal::Sieve::new(MAX + 1);
    let mut cum_primes: Vec<(usize, usize)> = Vec::new();

    let count_primes = |mut xs: Vec<(usize, usize)>| -> (usize, usize) {
        let mut sum = 0;
        for (i, x) in xs.clone().iter().enumerate() {
            sum += x.0;
            xs[i].1 = sum
        }
        let mut cnt = xs.len();
        let mut prime = 2;
        for i in (0..xs.len()).rev() {
            if sieve.is_prime(xs[i].1) {
                prime = xs[i].1;
                break;
            }
            cnt -= 1;
        }
        (prime, cnt)
    };

    cum_primes.push((2, 2));
    for i in 3..MAX + 1 {
        if sieve.is_prime(i) {
            let sum = cum_primes.last().unwrap().1 + i;
            if sum < MAX {
                cum_primes.push((i, sum))
            }
        }
    }

    let (mut max_prime, mut max_cnt) = (2, 0);
    while max_cnt < cum_primes.len() {
        let (prime, cnt) = count_primes(cum_primes.clone());
        if cnt > max_cnt {
            max_prime = prime;
            max_cnt = cnt
        }
        cum_primes.remove(0);
    }

    assert_eq!(max_prime, 997651);
    format!("p050 = {}", max_prime)
} // 997651

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (41,
     vec![p041, p042, p043, p044, p045, p046, p047, p048, p049, p050])
}
