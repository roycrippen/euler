//! Functions involving primes without dependencies.
//!
//!
//! # Examples
//!
//! ```
//! use euler_library::primes as eu_primes;
//!
//! assert_eq!(eu_primes::prime_factors(84), [2, 2, 3, 7]);
//! assert_eq!(eu_primes::prime_factors_unique(84), [2, 3, 7]);
//!
//! ```


/// Returns a vector of the prime factors n.
///
/// ```
/// use euler_library::primes as eu_primes;
///
/// assert_eq!(eu_primes::prime_factors(342), [2, 3, 3, 19]);
/// assert_eq!(eu_primes::prime_factors(123), [3, 41]);
/// ```
///
pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut xs: Vec<usize> = Vec::new();
    let mut i = 2;
    while n > 1 {
        while n % i == 0 {
            xs.push(i);
            n /= i;
        }
        i += 1;
    }
    xs
}

/// Returns a vector of the unique prime factors n.
///
/// ```
/// use euler_library::primes as eu_primes;
///
/// assert_eq!(eu_primes::prime_factors_unique(342), [2, 3, 19]);
/// assert_eq!(eu_primes::prime_factors_unique(123), [3, 41]);
/// ```
///
pub fn prime_factors_unique(n: usize) -> Vec<usize> {
    let mut xs = prime_factors(n);
    xs.dedup();
    xs
}


/// Returns the sum of unique prime factors of n.
///
/// ```
/// use euler_library::primes as eu_primes;
///
/// assert_eq!(eu_primes::sopf(342), 24);
/// assert_eq!(eu_primes::sopf(123), 44);
/// ```
///
pub fn sopf(n: usize) -> usize {
    prime_factors_unique(n).iter().fold(0, |acc, x| acc + x)
}


/// Return a vector of the count of unique prime factors of i in 0..n.
///
/// ```
/// use euler_library::primes as eu_primes;
///
/// assert_eq!(eu_primes::prime_factor_cnt(10), [0, 0, 1, 1, 1, 1, 2, 1, 1, 1]);
///
/// let prime_factor_cnt_90_to_99 =
///     eu_primes::prime_factor_cnt(100).into_iter().skip(90).take(10).collect::<Vec<_>>();
/// assert_eq!(prime_factor_cnt_90_to_99, [3, 2, 2, 2, 2, 2, 2, 1, 2, 2]);
/// ```
///
pub fn prime_factor_cnt(n: usize) -> Vec<usize> {
    let mut s = vec![0 as usize; n];
    for (i, _) in s.clone().iter().enumerate().skip(2) {
        if s[i] == 0 {
            let mut j = i;
            while j < n {
                s[j] += 1;
                j += i
            }
        }
    }
    s
}
