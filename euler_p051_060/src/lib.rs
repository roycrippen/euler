//! Project Euler solutions for problems 51 through 60.
//!
//! This crate is designed to be used via crate `euler`.

extern crate primal;

extern crate num;
use num::{BigUint, pow};
use num::bigint::ToBigUint;

extern crate euler_library;
use euler_library::common as eu;
use euler_library::cards::{Card, Hand, char_to_suit, char_to_val};

/// Prime digit replacements
pub fn p051() -> String {
    // assume solution set has:
    // 6 digits, 3 'same' digits that change,
    // ends in fixed number

    fn make_candidates(mask: [usize; 6], comb: Vec<usize>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        for i in 0..10 {
            let mut xs = comb.clone();
            xs.reverse();
            let mut ps: Vec<usize> = Vec::new();
            for v in &mask {
                if *v == 1 { ps.push(xs.pop().unwrap()) } else { ps.push(i) }
            }
            if ps[0] != 0 {
                res.push(to_usize(ps))
            }
        }
        res
    }

    fn to_usize(xs: Vec<usize>) -> usize {
        let mut res = 0;
        let mut fact = 1;
        for x in xs.iter().rev() {
            res += x * fact;
            fact *= 10;
        }
        res
    }

    let sieve = primal::Sieve::new(1_000_000);
    let combs = eu::perms_with_reps(3, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        .into_iter()
        .filter(|x| x[0] != 0 && x[2] % 2 != 0 && x[2] % 5 != 0)
        .collect::<Vec<_>>();

    let find_answer = |mask: &[usize; 6]| -> Option<usize> {
        let ys = combs.clone();
        for y in ys {
            let xs = y.clone();
            let cs = make_candidates(*mask, xs);
            if cs.iter().filter(|x| sieve.is_prime(**x)).count() == 8 {
                return Some(cs[0]);
            }
        }
        None
    };


    // 1's replaced by combinations; 0's by 0..10
    let masks = [[1, 1, 0, 0, 0, 1],
                 [1, 0, 1, 0, 0, 1],
                 [1, 0, 0, 1, 0, 1],
                 [1, 0, 0, 0, 1, 1],
                 [0, 1, 1, 0, 0, 1],
                 [0, 1, 0, 1, 0, 1],
                 [0, 1, 0, 0, 1, 1],
                 [0, 0, 1, 1, 0, 1],
                 [0, 0, 1, 0, 1, 1],
                 [0, 0, 0, 1, 1, 1]];

    let mut res = None;
    for mask in masks.iter().clone() {
        res = find_answer(mask);
        if res != None {
            break;
        }
    }

    format!("p051 = {}", res.unwrap())
} // 121313

/// Permuted multiples
pub fn p052() -> String {

    fn same_digits(a: usize, b: usize) -> bool {
        let mut xs = eu::to_bytes(a);
        xs.sort();
        let mut ys = eu::to_bytes(b);
        ys.sort();
        xs == ys
    }

    fn get_permuted(xs: Vec<usize>) -> Option<usize> {
        for v in xs {
            if !same_digits(v, 2 * v) {
                continue;
            }
            if !same_digits(v, 3 * v) {
                continue;
            }
            if !same_digits(v, 4 * v) {
                continue;
            }
            if !same_digits(v, 5 * v) {
                continue;
            }
            if same_digits(v, 6 * v) {
                return Some(v);
            }
        }
        None
    }

    let mut res = Some(0);
    let mut min = 1_000;
    let mut max = 10_000;
    for _ in 1..5 {
        let xs = (min..max).filter(|&x| x / min == 1).collect::<Vec<_>>();
        res = get_permuted(xs);
        if res != None {
            break;
        }
        min *= 10;
        max *= 10;
    }

    format!("p052 = {}", res.unwrap())
} // 142857

/// Combinatoric selections
pub fn p053() -> String {
    // combinations n C r
    fn ncr_recur(n: usize, r: usize) -> usize {
        if r == 0 { 1 } else { ncr_recur(n - 1, r - 1) * n / r }
    }

    // total available = 23+24+..+98+99+100 = 4797
    let mut cnt = 4797;
    for n in 23..101 {
        for r in 1.. {
            if ncr_recur(n, r) <= 1_000_000 { cnt -= 1 } else { break };
        }
        for r in (1..n + 1).rev() {
            if ncr_recur(n, r) <= 1_000_000 { cnt -= 1 } else { break };
        }
    }
    format!("p053 = {}", cnt)
} // 4075


/// Poker hands
pub fn p054() -> String {
    fn get_data() -> Vec<char> {
        let buffer = include_str!("../data/p054_hands.txt");
        buffer.chars().filter(|&x| x != ' ' && x != '\n').collect::<Vec<char>>()
    }

    fn get_hand(mut cs: Vec<char>) -> Hand {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..5 {
            let suit = char_to_suit(cs.pop().unwrap());
            let val = char_to_val(cs.pop().unwrap());
            cards.push(Card { suit: suit, val: val })
        }
        Hand { cards: cards }
    }

    fn get_hands() -> Vec<Hand> {
        let hands_chars = get_data();
        let mut cs: Vec<char> = Vec::new();
        let mut hands: Vec<Hand> = Vec::new();
        for v in hands_chars {
            cs.push(v);
            if cs.len() == 10 {
                hands.push(get_hand(cs.clone()));
                cs.clear();
            }
        }
        hands.reverse();
        hands
    }

    // main program
    let mut hands = get_hands();
    let mut cnt = 0;
    while hands.len() > 1 {
        let a = hands.pop().unwrap();
        let b = hands.pop().unwrap();
        let ra = a.get_rank();
        let rb = b.get_rank();
        if ra > rb {
            cnt += 1;
        }
    }
    assert_eq!(cnt, 376);
    format!("p054 = {}", cnt)
} // 376

/// Lychrel numbers
pub fn p055() -> String {
    fn is_lychrel(n: usize) -> bool {
        let mut x: BigUint = n.to_biguint().unwrap().clone();
        for _ in 1..50 {
            let mut xs = eu::to_bytes(x.clone());
            xs.reverse();
            let t: BigUint = eu::from_bytes(&xs).unwrap();
            x = x + t;
            if eu::is_palindrome(x.clone()) {
                return false;
            }
        }
        true
    }

    let cnt = (1..10_000).fold(0, |acc, x| if is_lychrel(x) { acc + 1 } else { acc });
    format!("p055 = {}", cnt)
} // 249

/// Powerful digit sum
pub fn p056() -> String {
    let mut max = 0;
    for i in (90..100).rev() {
        for b in (90..100).rev() {
            let a = i.to_biguint().unwrap();
            let digs = eu::to_bytes(pow(a.clone(), b));
            let cnt = digs.iter().fold(0, |acc, x| acc + (*x - 48) as usize);
            if cnt > max {
                max = cnt
            }
        }

    }
    format!("p056 = {}", max)
} // 972

/// Square root convergents
pub fn p057() -> String {
    let mut cnt = 0;
    let mut n = 1.to_biguint().unwrap();
    let mut d = 2.to_biguint().unwrap();;
    for _ in 1..1000 {
        if (&n + &d).to_string().len() > d.to_string().len() {
            cnt += 1;
        }
        let new_n = d.clone();
        d = 2.to_biguint().unwrap() * d + n;
        n = new_n;
    }


    format!("p057 = {}", cnt)
} // 153

/// Spiral primes
pub fn p058() -> String {
    let get_result = || -> usize {
        let mut prime_cnt = 0;
        let mut factor = 2;
        let mut v = 1;
        loop {
            for _ in 0..4 {
                if primal::is_prime(v) {
                    prime_cnt += 1;
                }
                v += factor;
            }
            factor += 2;
            let side = (v as f32).sqrt();
            let tot_diag = (2.0 * side - 1.0) * 0.1;
            if (prime_cnt as f32) < tot_diag {
                return side as usize;
            }
        }
    };

    let res = get_result();
    assert_eq!(res, 26241);
    format!("p058 = {}", res)
} // 26241

/// XOR decryption
pub fn p059() -> String {
    fn decode(msg: &[u8], key: &[u8]) -> Vec<u8> {
        msg.iter()
            .zip(key.iter()
                .cycle()
                .take(msg.len()))
            .map(|(&a, b)| a ^ b)
            .collect::<Vec<u8>>()
    }

    fn get_msg() -> Vec<u8> {
        let buffer = include_str!("../data/p059_cipher.txt")
            .chars()
            .filter(|&x| x != '\"' && x != '\n')
            .collect::<String>();

        buffer.split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect()
    }

    let comb = eu::perms_without_reps_recur(3, &"abcdefghijklmnopqrstuvwxyz".to_string().into_bytes());
    let msg = get_msg();
    let mut res: usize = 0;
    for v in comb {
        let decrypted = decode(&msg, &v);
        let cnt = decrypted.iter().fold(0, |acc, x| if *x == 32 { acc + 1 } else { acc });
        if cnt >= 200 {
            res = decrypted.iter().fold(0, |acc, x| acc + *x as usize);
            break;
        }
    }
    assert_eq!(res, 107359);
    format!("p059 = {}", res)
} // 107359

/// Prime pair sets
pub fn p060() -> String {
    fn eval(w: &[usize], k: usize, s: usize, sieve: &primal::Sieve) -> usize {
        let ok = |a, b| {
            let mut ten = 1;
            while ten <= b {
                ten *= 10;
            }
            sieve.is_prime(ten * a + b)
        };

        if k == 0 {
            return s;
        }

        let mut sum = 0;
        for (i, &wi) in w.iter().enumerate() {
            let w2 = w[i + 1..]
                .iter()
                .cloned()
                .filter(|&wj| ok(wi, wj) && ok(wj, wi))
                .collect::<Vec<_>>();
            sum = eval(&w2, k - 1, s + wi, sieve);
            if sum != 0 {
                break;
            }
        }
        sum
    }

    let sieve = primal::Sieve::new(100_000_005);
    let some_primes = sieve.primes_from(0)
        .take_while(|&p| p < 10_000)
        .collect::<Vec<_>>();

    let sum = eval(&some_primes, 5, 0, &sieve);
    format!("p060 = {}", sum)
} // 26033

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (51, vec![p051, p052, p053, p054, p055, p056, p057, p058, p059, p060])
}
