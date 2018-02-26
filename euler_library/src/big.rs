//!   Library used in solving Project Euler problems needing `BigInt` or `BigUint`.
//!
//!
//! # Example
//!
//! ```
//! use euler_library::big as eu_big;
//!
//! let big_number_string = eu_big::factorial(31).to_string();
//! assert_eq!(big_number_string.to_string(), "8222838654177922817725562880000000");
//! ```

extern crate num;
use self::num::{BigInt, BigUint, One, Zero, pow};
use self::num::bigint::ToBigUint;

/// Returns n factorial as a `BigUint`.
///
/// ```
/// use euler_library::big as eu_big;
///
/// let big_number_string = eu_big::factorial(31).to_string();
/// assert_eq!(big_number_string.to_string(), "8222838654177922817725562880000000");
/// ```
pub fn factorial(n: usize) -> BigUint {
    let mut fact: BigUint = One::one();
    if n > 1 {
        for i in 2..n + 1 {
            fact = fact * i.to_biguint().unwrap()
        }
    }
    fact
}

/// Returns `BigUnit` square root of usize n to digits precision.
///
/// Function does not tell you where decimal is.
/// For example square root of 2 is 1.1414213..., function return 11414213...
///
/// ```
/// use euler_library::big as eu_big;
///
/// let sqrt_2_string = eu_big::precision_sqrt(2, 40).to_string();
/// assert_eq!(sqrt_2_string.to_string(), "1414213562373095048801688724209698078569");
/// ```
pub fn precision_sqrt(n: usize, digits: usize) -> BigUint {
    let ten = &10.to_biguint().unwrap();
    let hundred = &(ten * ten);
    let limit = pow(ten.clone(), digits + 1);
    let mut a = (5 * n).to_biguint().unwrap();
    let mut b = 5.to_biguint().unwrap();
    while b < limit {
        if a >= b {
            a = a.clone() - b.clone();
            b = b.clone() + ten;
        } else {
            a = a.clone() * hundred;
            b = (b.clone() / ten) * hundred + 5.to_biguint().unwrap();
        }
    }
    b / hundred
}

/// Return (numerator, denominator) after evaluating `continued_fraction`.
///
/// Form of continued fraction: (a0, [t1, t2, t3, ....]).
/// see `euler_library::common::continued_fraction` to generate a periodic continued fraction.
/// `http://roycrippen.github.io/euler_library/euler_library/common/fn.continued_fraction.html`
///
/// ```
/// use euler_library::big as eu_big;
///
/// // periodic continued fraction for 23 = (4, [1,3,1,8])
/// let (a0, mut ts) = (4, vec![1, 3, 1, 8]);
/// ts = ts.clone().into_iter().cycle().take(64).collect::<Vec<usize>>();
///
/// let (numerator, denominator) = eu_big::continued_fraction(a0, ts.clone());
/// assert_eq!(numerator.to_string(), "3468077590434524694871282564");
/// assert_eq!(denominator.to_string(), "723144166673926627543073281");
/// ```
pub fn continued_fraction(a0: usize, mut xs: Vec<usize>) -> (BigUint, BigUint) {
    fn go(n: &BigUint, d: &BigUint, mut xs: Vec<BigUint>) -> (BigUint, BigUint) {
        if xs.is_empty() {
            return (n.clone(), d.clone());
        }
        let a = xs.pop().unwrap();
        go(&(a * n + d), n, xs)
    }

    xs.insert(0, a0);
    let a = xs.pop().unwrap().to_biguint().unwrap();
    let xs = xs.into_iter().map(|x| x.to_biguint().unwrap()).collect::<Vec<BigUint>>();
    go(&a, &1.to_biguint().unwrap(), xs)
}

/// Returns a `BigInt` vec[p(0), p(1)...p(n)] generating partition function
///
/// http://oeis.org/A000041/list
///
/// ```
/// use euler_library::big as eu_big;
///
/// let ps = eu_big::integer_partitions(1_000).iter().map(|x| x.to_string()).collect::<Vec<_>>();
/// assert_eq!(&ps[23..26], ["1255", "1575", "1958"]);
/// assert_eq!(&ps[1000..], ["24061467864032622473692149727991"]);
/// ```
pub fn integer_partitions(n: usize) -> Vec<BigInt> {
    let k = (1..n)
        .flat_map(|i| vec![i * (3 * i - 1) / 2, i * (3 * i - 1) / 2 + i])
        .collect::<Vec<_>>();

    let one: BigInt = One::one();
    let mut p: Vec<BigInt> = vec![one.clone()];
    let sign: Vec<BigInt> = vec![one.clone(), one.clone(), -one.clone(), -one.clone()];

    for i in 1..n + 1 {
        let mut temp: BigInt = Zero::zero();
        let mut j = 0;
        while k[j] <= i {
            temp = temp + (p[i - k[j]].clone()) * sign[j % 4].clone();
            j += 1;
        }
        p.push(temp)
    }
    p
}
