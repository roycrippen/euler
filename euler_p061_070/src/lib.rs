//! Project Euler solutions for problems 61 through 70.
//!
//! This crate is designed to be used via crate `euler`.

use std::cmp;

extern crate primal;

extern crate itertools;
use itertools::Itertools;

extern crate num;
use num::BigUint;
use num::bigint::ToBigUint;

extern crate euler_library;
use euler_library::common as eu;
use euler_library::big as eu_big;

/// Cyclical figurate numbers
pub fn p061() -> String {
    fn is_cyclic(left: usize, right: usize) -> bool {
        right / 100 == left % 100
    }

    fn get_polygonals() -> Vec<Vec<usize>> {
        fn f3(n: usize) -> usize {
            n * (n + 1) / 2
        };
        fn f4(n: usize) -> usize {
            n * n
        };
        fn f5(n: usize) -> usize {
            n * (3 * n - 1) / 2
        };
        fn f6(n: usize) -> usize {
            n * (2 * n - 1)
        };
        fn f7(n: usize) -> usize {
            n * (5 * n - 3) / 2
        };
        fn f8(n: usize) -> usize {
            n * (3 * n - 2)
        };
        let fs: Vec<fn(usize) -> usize> = vec![f3, f4, f5, f6, f7, f8];

        fs.iter()
          .map(|f| {
              (1..)
                  .map(f)
                  .skip_while(|&p| p < 999)
                  .take_while(|&p| p < 10000)
                  .collect::<Vec<_>>()
          })
          .collect::<Vec<Vec<_>>>()
    }

    fn add_to_set(list: Vec<Vec<usize>>, vs: Vec<usize>) -> Vec<Vec<usize>> {
        if list.is_empty() {
            return vs.iter().map(|&x| vec![x]).collect();;
        }
        let mut xss: Vec<Vec<usize>> = Vec::new();
        for ls in list {
            for v in vs.clone() {
                if is_cyclic(ls[ls.len() - 1], v) {
                    let mut x = ls.clone();
                    x.push(v);
                    xss.push(x);
                }
            }
        }
        xss
    }

    fn eval(pss: Vec<Vec<usize>>) -> usize {
        let p = &vec![0, 1, 2, 3, 4, 5];
        let perms = eu::perms_without_reps_recur(p.len(), p);
        for perm in perms {
            let mut list: Vec<Vec<usize>> = Vec::new();
            for p in perm {
                list = add_to_set(list, pss[p].clone());
            }
            let res = list.iter()
                          .filter(|xs| is_cyclic(xs[xs.len() - 1], xs[0]))
                          .collect::<Vec<_>>();
            if res.len() == 1 && res[0].len() == p.len() {
                // println!("{:?}", res);
                return res[0].iter().fold(0, |acc, x| acc + x);
            }
        }
        0
    }

    let res = eval(get_polygonals());
    assert_eq!(res, 28684);
    format!("p061 = {}", eval(get_polygonals()))
} // 28684

/// Cubic permutations
pub fn p062() -> String {
    let mut xs: Vec<(usize, usize)> = Vec::new();
    for i in 300..10000 {
        let cube = (i as f64).powf(3.) as usize;
        let mut ds = eu::to_bytes(cube);
        ds.sort();
        // pad front of number with '9' so '0's not lost
        ds.insert(0, 57 as u8);
        xs.push((eu::from_bytes(&ds).unwrap(), cube));
    }

    xs.sort();
    let ys = xs.into_iter()
               .group_by(|&(k, _)| k)
               .filter(|&(_, ref group)| group.len() == 5)
               .collect::<Vec<_>>();

    let mut res: usize = 0;
    if !ys.is_empty() {
        // println!("{:?}", ys[0]);
        res = ys[0].1[0].1
    }

    assert_eq!(res, 127035954683);
    format!("p062 = {}", res)
} // 127035954683 == 5027^3

/// Powerful digit counts
pub fn p063() -> String {
    let mut cnt = 1;
    for m in 1..11 {
        for n in 1..100 {
            let l = ((m as f64).powf(n as f64) as usize).to_string().len();
            if l == n {
                cnt += 1;
            }
        }
    }

    assert_eq!(cnt, 49);
    format!("p063 = {}", cnt)
} // 49

/// Odd period square roots
pub fn p064() -> String {
    fn is_odd_period(n: u32) -> bool {
        let a0 = (n as f32).sqrt() as u32;
        if a0 * a0 == n {
            return false;
        }
        let mut period = 0;
        let mut d = 1;
        let mut m = 0;
        let mut a = a0;
        while a != 2 * a0 {
            m = d * a - m;
            d = (n - m * m) / d;
            a = (a0 + m) / d;
            period += 1;
        }
        period % 2 == 1
    }

    let cnt = (1..10001).fold(0, |acc, n| if is_odd_period(n) { acc + 1 } else { acc });
    assert_eq!(cnt, 1322);
    format!("p064 = {}", cnt)
} // 1322

/// Convergents of e
pub fn p065() -> String {
    let e = [1].iter()
               .cycle()
               .enumerate()
               .skip(2)
               .map(|(idx, &x)| if idx % 3 == 0 { idx / 3 * 2 } else { x })
               .take(99)
               .collect::<Vec<_>>();

    let (n, _) = eu_big::continued_fraction(2, e);
    // println!("numerator = {}", n.clone().to_string());
    // println!("denominator = {}", d.clone().to_string());
    let sum = eu::to_bytes(n)
                  .iter()
                  .fold(0 as u32, |acc, &x| acc + (x as u32) - 48);

    assert_eq!(sum, 272);
    format!("p065 = {}", sum)
} // 272

/// Diophantine equation
pub fn p066() -> String {
    // pell's equation x^2 -Dy^2
    // sqrt(D) expansion and test
    fn pell_min(d: usize) -> BigUint {
        let d_big = d.to_biguint().unwrap();
        let sqrt = eu::sqrt_terms(d);
        if sqrt == None {
            return 0.to_biguint().unwrap();
        }
        let (base, repeat) = sqrt.unwrap();
        let mut ys_iter = repeat.iter().cycle();
        let mut ys: Vec<usize> = Vec::new();
        let one = 1.to_biguint().unwrap();
        loop {
            ys.push(*ys_iter.next().unwrap());
            let (num, den) = eu_big::continued_fraction(base, ys.clone());
            if &num * &num == &d_big * &den * &den + &one {
                return num;
            }
        }
    }

    assert!(pell_min(61).to_string() == 1766319049.to_string());
    assert!(pell_min(67).to_string() == 48842.to_string());

    let mut max = (0, 1.to_biguint().unwrap());
    // https://oeis.org/A033316
    // largest D for min x < 542
    for i in 541..1001 {
        let val = pell_min(i);
        if val > max.1 {
            max.0 = i;
            max.1 = val
        }

    }

    assert_eq!(max.0, 661);
    format!("p066 = {}", max.0)
} // 661

/// Maximum path sum II
pub fn p067() -> String {
    fn get_data() -> Vec<Vec<u32>> {
        let buffer = include_str!("../data/p067_triangle.txt");
        buffer.lines()
              .map(|x| {
                  x.split(' ')
                   .map(|x| x.parse().unwrap())
                   .collect::<Vec<u32>>()
              })
              .collect()
    }

    let mut xss = get_data();
    for i in (0..xss.len()).rev() {
        for j in 0..i {
            xss[i - 1][j] += cmp::max(xss[i][j], xss[i][j + 1])
        }
    }

    assert_eq!(xss[0][0], 7273);
    format!("p067 = {}", xss[0][0])
} // 7273

/// Magic 5-gon ring
pub fn p068() -> String {

    #[derive(Debug, Clone, PartialEq, Ord, Eq, PartialOrd)]
    struct Ring {
        orig_ord: u32,
        sum: u32,
        tup: (u32, u32, u32),
    }

    fn ring_tup(xs: &[u32]) -> Vec<Ring> {
        let mut res: Vec<Ring> = Vec::new();
        for (i, _) in xs.iter().enumerate().take(xs.len() - 1) {
            // for i in 0..xs.len() - 1 {
            let t = Ring { orig_ord: (i) as u32, tup: (0, xs[i], xs[i + 1]), sum: xs[i] + xs[i + 1] };
            res.push(t);
        }
        let t = Ring {
            orig_ord: xs.len() as u32 - 1,
            tup: (0, xs[xs.len() - 1], xs[0]),
            sum: xs[0] + xs[xs.len() - 1],
        };
        res.push(t);
        res = res.into_iter()
                 .sorted_by(|a, b| Ord::cmp(&b.sum, &a.sum))
                 .into_iter()
                 .collect::<Vec<Ring>>();
        res
    }

    fn is_valid_set(rings: &[Ring], external: &[u32]) -> bool {
        let sum = rings[0].sum + external[0];
        for (i, ring) in rings.iter().enumerate() {
            if sum != ring.sum + external[i] {
                return false;
            }
        }
        true
    }

    fn vect_to_string<T: ToString>(xs: Vec<T>) -> String {
        xs.iter().fold("".to_string(), |acc, x| acc + &x.to_string())
    }

    // values 6,7,8,9,10 must be in outer ring
    let pool = vec![1 as u32, 2, 3, 4, 5];
    let perms = eu::perms_without_reps_recur(5, &pool.clone());

    let mut ress: Vec<String> = Vec::new();
    for ps in perms {
        let mut rings = ring_tup(&ps);
        let external = vec![6, 7, 8, 9, 10];
        if is_valid_set(&rings, &external) {
            for (i, ring) in rings.iter_mut().enumerate() {
                ring.tup.0 = external[i]
            }
            rings = rings.into_iter()
                         .sorted_by(|a, b| Ord::cmp(&a.tup, &b.tup))
                         .into_iter()
                         .collect::<Vec<Ring>>();
            for (i, ring) in rings.clone().iter().enumerate() {
                if ring.orig_ord < rings[0].orig_ord {
                    rings[i].orig_ord += 10;
                }
            }
            rings = rings.into_iter()
                         .sorted_by(|a, b| Ord::cmp(&a.orig_ord, &b.orig_ord))
                         .into_iter()
                         .collect::<Vec<Ring>>();
            let mut term: Vec<u32> = Vec::new();
            let mut str = "".to_string();
            for ring in rings {
                term.push(ring.tup.0);
                term.push(ring.tup.1);
                term.push(ring.tup.2);
                str = str + &vect_to_string(term.clone());
                term.clear();
            }
            ress.push(str)
        }
    }

    ress.sort();
    let res = ress.last().unwrap();
    assert_eq!(*res, "6531031914842725".to_string());
    format!("p068 = {}", res)
} // 6531031914842725

/// Totient maximum
pub fn p069() -> String {
    let mut max = 1.0;
    let mut idx = 1;
    for (i, x) in eu::phis(1_000_000).iter().enumerate().skip(1) {
        if i as f64 / *x as f64 > max {
            max = i as f64 / *x as f64;
            idx = i;
        }
    }

    assert_eq!(idx, 510510);
    format!("p069 = {}", idx)
} // 510510

/// Totient permutation
pub fn p070() -> String {
    let primes = primal::Primes::all().take(10_000).collect::<Vec<_>>();

    let mut best = 1;
    let mut best_ratio: f64 = 1_000.0;
    let limit = 10_000_000;
    for (i, vi) in primes.iter().enumerate().take(500).skip(1) {
        for vj in primes.iter().take(500).skip(i + 1) {
            let n = vi * vj;
            if n > limit {
                break;
            }
            let phi = (vi - 1) * (vj - 1);
            let ratio = n as f64 / phi as f64;
            if best_ratio > ratio && eu::is_perm(n, phi) {
                best = n;
                best_ratio = ratio;
            }
        }
    }

    assert_eq!(best, 8319823);
    format!("p070 = {}", best)
} // 8319823

/// Totient permutation
// first attempt; works, uses phis function and straight forward
// but quite a bit slower
pub fn p070a() -> String {
    let mut best = 1;
    let mut best_ratio: f64 = 1_000.0;
    for (n, phi) in eu::phis(10_000_000).into_iter().enumerate().skip(8_000_000) {
        let ratio = n as f64 / phi as f64;
        if best_ratio > ratio && eu::is_perm(n, phi) {
            best = n;
            best_ratio = ratio;
        }
    }

    assert_eq!(best, 8319823);
    format!("p070a = {}", best)
} // 8319823

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (61,
     vec![p061, p062, p063, p064, p065, p066, p067, p068, p069, p070])
}
