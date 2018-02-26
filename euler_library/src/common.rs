//! Common functions used in solving Project Euler problems.
//!
//!
//! # Examples
//!
//! ```
//! use euler_library::common as eu;
//!
//! assert!(eu::is_palindrome(12321));
//! assert_eq!(eu::divisor_sum_list(10), [0, 0, 1, 1, 3, 1, 6, 1, 7, 4, 8]);
//!
//! let res = [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]];
//! assert_eq!(eu::perms_with_reps(2, &vec![1, 2, 3]), res);
//! ```

use std::fmt::Debug;
use std::iter;
use std::iter::{Repeat, Take};
use std::str;
use std::str::FromStr;

/// Returns the sum of the proper divisors of n (not including n).
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::divisor_sum(10), 8);
/// ```
pub fn divisor_sum(n: usize) -> usize {
    match n {
        0 | 1 => 0,
        _ => {
            let max = ((n as f64).sqrt() + 1.0) as usize;
            (2..max).fold(1, |acc, x| {
                if n % x == 0 {
                    let d = n / x;
                    if d == x { acc + d } else { acc + x + d }
                } else {
                    acc
                }
            })
        }
    }
}

/// Returns a vector containing `divisor_sum`(i) for i from 0 to n.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::divisor_sum_list(10), [0, 0, 1, 1, 3, 1, 6, 1, 7, 4, 8]);
/// assert_eq!(*eu::divisor_sum_list(10).last().unwrap(), eu::divisor_sum(10));
/// ```
pub fn divisor_sum_list(limit: usize) -> Vec<usize> {
    let mut xs = vec![0; limit + 1];
    for i in 1..limit / 2 + 1 {
        let mut j = 2 * i;
        while j <= limit {
            xs[j] += i;
            j += i
        }
    }
    xs
}

/// Returns true if string is pandigital over digits start to n.
///
/// Normal definition is that an n-digit number is pandigital if it makes
/// use of all the digits 1 to n exactly once.
/// This function checks if pandigital from start to n exactly once.
///
/// ```
/// use euler_library::common as eu;
///
/// assert!(eu::is_pandigital("456123".to_string(), 1));
/// assert!(eu::is_pandigital("4560123".to_string(), 0));
/// ```
pub fn is_pandigital(string: String, start: usize) -> bool {
    let bs = string.as_bytes();
    let cv = 48 + start;
    for v in cv..string.len() + cv {
        if !bs.contains(&(v as u8)) {
            return false;
        }
    }
    true
}

/// Returns the sum of the digits of a string.
///
/// Panics if string s can not be cast as usize.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::sum_of_digits("123".to_string()), 6);
/// ```
pub fn sum_of_digits(s: String) -> usize {
    s.chars().into_iter().fold(0, |acc, x| acc + x.to_digit(10).unwrap()) as usize
}

/// Returns n as a vector of bytes.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::to_bytes(123), [49, 50, 51]);
/// ```
pub fn to_bytes<T: ToString>(n: T) -> Vec<u8> {
    n.to_string().into_bytes()
}


/// Returns Result from attempting to parses a Vec of u8.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::from_bytes::<usize>(&[51, 50, 49]), Ok(321 as usize));
///
/// let bytes = eu::to_bytes(123);
/// assert_eq!(eu::from_bytes(&bytes), Ok(123));
/// ```
pub fn from_bytes<T>(vec: &[u8]) -> Result<T, T::Err>
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
    str::from_utf8(vec).unwrap().parse::<_>()
}

/// Returns usize n as a vector of its usize digits.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::to_digits(123), [1, 2, 3]);
/// ```
pub fn to_digits(mut n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    while n != 0 {
        res.push(n % 10);
        n /= 10
    }
    res.reverse();
    res
}

/// Returns a usize number from vector xs of usize digits.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::from_digits(&vec![1,2,3]), 123);
/// ```
pub fn from_digits(xs: &[usize]) -> usize {
    let mut res = 0;
    for elt in xs {
        res = res * 10 + elt
    }
    res
}

/// Returns true if value v is a palindrome (reads the same backward or forward).
///
/// ```
/// use euler_library::common as eu;
///
/// assert!(eu::is_palindrome(12321));
/// assert!(eu::is_palindrome("abcba"));
///
/// ```
pub fn is_palindrome<T: ToString>(n: T) -> bool {
    let cs = to_bytes(n);
    if *cs.first().unwrap() != *cs.last().unwrap() {
        return false;
    }
    let mut rev_cs = cs.clone();
    rev_cs.reverse();
    rev_cs == cs
}


/// Returns factorial of n for numbers less than 21.
///
/// Panics for n above 20.  Big factorial implemented in `eu_big` crate.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::factorial(15), 1307674368000);
///
/// ```
pub fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |p, n| p * n)
}

/// Returns permutations k chosen from xs, odered, repetition allowed.
///
/// Iterative solution.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::perms_with_reps(2, &vec![1,2]), [[1,1], [1,2], [2,1], [2,2]]);
///
/// ```
pub fn perms_with_reps<T>(k: usize, xs: &[T]) -> Vec<Vec<T>>
    where T: Clone
{
    let vec = replicate(k, xs.to_vec()).collect::<Vec<_>>();
    cartesian_product(&vec)
}

/// Returns and iterator of length n of repeated values of elt
///
/// ```
/// use euler_library::common as eu;
///
/// let repeated = eu::replicate(2, vec![1, 2, 3]);
/// assert_eq!(repeated.collect::<Vec<_>>(), [[1, 2, 3], [1, 2, 3]]);
///
/// let repeated = eu::replicate(3, "abc").collect::<String>();
/// assert_eq!(repeated, "abcabcabc");
/// ```
pub fn replicate<T>(n: usize, elt: T) -> Take<Repeat<T>>
    where T: Clone
{
    iter::repeat(elt).take(n)
}

/// Returns the cartesian product of a Vec of Vec of T.
///
/// Recursive solution.
///
/// ```
/// use euler_library::common as eu;
///
/// let xss = vec![vec![1, 2], vec![3, 4]];
/// assert_eq!(eu::cartesian_product(&xss), [[1, 3], [1, 4], [2, 3], [2, 4]]);
///
/// ```
pub fn cartesian_product<T: Clone>(lists: &[Vec<T>]) -> Vec<Vec<T>> {
    fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
        a.into_iter()
            .flat_map(|xs| {
                b.iter()
                    .cloned()
                    .map(|y| {
                        let mut temp = xs.clone();
                        temp.push(y);
                        temp
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    match lists.split_first() {
        Some((first, rest)) => {
            let init = first.into_iter()
                .clone()
                .map(|n| vec![n.clone()])
                .collect::<Vec<Vec<T>>>();
            rest.iter()
                .clone()
                .fold(init, |vec, list| partial_cartesian(vec, list))
        }
        None => vec![],
    }
}

/// Returns permutations k chosen from xs, odered, no repetition.
///
/// Recursive solution.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::perms_without_reps_recur(2, &[1,2]), [[1,2], [2,1]]);
///
/// ```
pub fn perms_without_reps_recur<T>(k: usize, xs: &[T]) -> Vec<Vec<T>>
    where T: Clone + PartialEq + Ord + Debug
{
    match k {
        0 => return vec![vec![]],
        1 => xs.iter().map(|x| vec![x.clone()]).collect(),
        _ => {
            let mut list: Vec<Vec<T>> = Vec::new();
            for x in xs {
                let ts = perms_without_reps_recur(k - 1, xs);
                for mut t in ts {
                    if !t.contains(x) {
                        t.push(x.clone());
                        list.push(t);
                    }
                }
            }
            list.sort();
            list
        }
    }
}

/// Returns k nested 'loops' from xs, ordered.
///
/// `k_nested_recur`(3, &[1,2]) sudo code same as:
///
///  for (i,vi) 0..2 { for (j,vj) in i..2 { for (k,vk) in j..2 { list.push(vi, vj, vk) } } }
///
/// Recursive solution.
///
/// ```
/// use euler_library::common as eu;
///
/// let res = [[1,1,1], [1,1,2], [1,2,1], [1,2,2], [2,1,1], [2,1,2], [2,2,1], [2,2,2]];
/// assert_eq!(eu::k_nested_recur(3, &[1,2]), res);
///
/// let xss = eu::k_nested_recur(8, &["red", "green", "blue", "orange"]);
/// assert_eq!(xss.len(), 65536);
///
/// ```
pub fn k_nested_recur<T>(k: usize, xs: &[T]) -> Vec<Vec<T>>
    where T: Clone + PartialEq + Ord + Debug
{
    match k {
        0 => return vec![vec![]],
        1 => xs.iter().map(|x| vec![x.clone()]).collect(),
        _ => {
            let mut list: Vec<Vec<T>> = Vec::new();
            for x in xs {
                let ts = k_nested_recur(k - 1, xs);
                for mut t in ts {
                    t.push(x.clone());
                    list.push(t);
                }
            }
            list.sort();
            list
        }
    }
}

/// Returns continued fraction form of sqrt of n.
///
/// Return None if perfect square.
///
/// Form: (a0, [t1, t2..., tn]), see https://projecteuler.net/problem=64
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::sqrt_terms(13), Some((3, vec![1, 1, 1, 1, 6])));
/// assert_eq!(eu::sqrt_terms(25), None);
///
/// ```
pub fn sqrt_terms(n: usize) -> Option<(usize, Vec<usize>)> {
    let a0 = (n as f32).sqrt() as usize;
    if a0 * a0 == n {
        return None;
    }
    let mut xs: Vec<usize> = Vec::new();
    let mut d = 1;
    let mut m = 0;
    let mut a = a0;
    while a != 2 * a0 {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        xs.push(a)
    }
    Some((a0, xs))
}

/// Return (numerator, denominator) after evaluating `continued_fraction`.
///
/// Form of continued fraction: (a0, [t1, t2, t3, ....])
///
/// ```
/// use euler_library::common as eu;
///
/// let root_13 = eu::sqrt_terms(13);
///
/// assert_eq!(root_13, Some((3, vec![1, 1, 1, 1, 6])));
///
/// let (a0, mut ts) = root_13.unwrap();
/// ts = ts.clone().into_iter().cycle().take(15).collect::<Vec<usize>>();
///
/// assert_eq!(ts, [1, 1, 1, 1, 6, 1, 1, 1, 1, 6, 1, 1, 1, 1, 6]);
///
/// assert_eq!(eu::continued_fraction(a0, ts), (154451, 42837));
/// ```
pub fn continued_fraction(a0: usize, mut xs: Vec<usize>) -> (usize, usize) {
    fn go(n: usize, d: usize, mut xs: Vec<usize>) -> (usize, usize) {
        if xs.is_empty() {
            return (n, d);
        }
        let a = xs.pop().unwrap();
        go(a * n + d, n, xs)
    }

    xs.insert(0, a0);
    let a = xs.pop().unwrap();
    go(a, 1, xs)
}

/// Returns true if a and b are permutations of each other.
///
/// ```
/// use euler_library::common as eu;
///
/// assert!(eu::is_perm(123,231));
/// assert!(eu::is_perm("yes","esy"));
/// ```
pub fn is_perm<T: ToString>(a: T, b: T) -> bool {
    let mut a_buf = to_bytes(a);
    a_buf.sort();
    let mut b_buf = to_bytes(b);
    b_buf.sort();
    a_buf == b_buf
}

/// Returns vector phi values (also called totient) from 0 to d.
///
/// Euler's totient function counts the positive integers up to a
/// given number n that are relatively prime to n.
///
/// ```
/// use euler_library::common as eu;
///
/// let phi_90_to_100 = eu::phis(100).into_iter().skip(90).collect::<Vec<_>>();
///
/// assert_eq!(phi_90_to_100, [24, 72, 44, 60, 46, 72, 32, 96, 42, 60, 40]);
/// ```
pub fn phis(d: usize) -> Vec<usize> {
    let n = d + 1;
    let mut phi = vec![0; n];
    phi[1] = 1;
    let mut idx = 2;
    while idx < n {
        if phi[idx] == 0 {
            phi[idx] = idx - 1;
            let mut jj = 2;
            while jj * idx < n {
                if phi[jj] != 0 {
                    let mut q = jj;
                    let mut f = idx - 1;
                    while q % idx == 0 {
                        f *= idx;
                        q /= idx;
                    }
                    phi[idx * jj] = f * phi[q];
                }
                jj += 1
            }
        }
        idx += 1;
    }
    phi
}

/// Returns vector of the running total of xs.
///
/// ```
/// use euler_library::common as eu;
///
/// assert_eq!(eu::accumulate(&[1,2,3,4]), [1,3,6,10]);
/// assert_eq!(eu::accumulate(&[1,1,1,1,1]), [1,2,3,4,5]);
/// ```
pub fn accumulate(xs: &[usize]) -> Vec<usize> {
    xs.into_iter()
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .collect()
}
