//! Project Euler solutions for problems 71 through 80.

extern crate euler_library;
use self::euler_library::common as eu;
use self::euler_library::primes;
use self::euler_library::big as eu_big;

/// Ordered fractions
pub fn eu071() -> String {
    let (mut answer, mut c) = (2, 5);
    let (b, d) = (3, 7);
    while c + d <= 1_000_000 {
        answer += b;
        c += d;
    }
    format!("eu071 = {}", answer)
} // 428570

/// Counting fractions
pub fn eu072() -> String {
    let sum = eu::phis(1_000_000).iter().fold(0, |acc, x| acc + x);
    format!("eu072 = {}", sum - 1)
} // 303963552391

/// Counting fractions in a range
pub fn eu073() -> String {
    let phi = eu::phis(12_000);
    let c = phi.iter().skip(6).fold(0, |acc, x| acc + x);
    // next line logic: c*1/2 - c*1/3 == c/6
    let res = c / 6;
    format!("eu073 = {}", res - 2)
} // 7295372

/// Digit factorial chains
pub fn eu074() -> String {
    static FACT_SMALL: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    fn fact_sum(n: usize) -> usize {
        let mut temp = n;
        let mut sum = 0;
        while temp > 0 {
            sum += FACT_SMALL[temp % 10];
            temp /= 10;
        }
        sum
    }

    let mut cache: Vec<usize> = vec![0; 1_000_000];
    cache[0] = 2;
    cache[1] = 2;
    cache[2] = 1;
    cache[145] = 1;
    cache[169] = 3;
    cache[871] = 2;
    cache[872] = 2;
    cache[40585] = 1;
    cache[363601] = 3;
    cache[45361] = 2;
    cache[45362] = 2;
    cache[1454] = 3;

    let mut fact_non_repeat_cnt = |n: usize| -> usize {
        let mut cnt = 0;
        let mut term = n;
        loop {
            term = fact_sum(term);
            cnt += 1;
            if term < 1_000_000 && cache[term] != 0 {
                cache[n] = cache[term] + cnt;
                return cache[term] + cnt;
            }
        }
    };

    let sum = (3..1_000_000).fold(0,
                                  |acc, x| if fact_non_repeat_cnt(x) == 60 { acc + 1 } else { acc });
    assert_eq!(sum, 402);
    format!("eu074 = {}", sum)
} // 402

/// Singular integer right triangles
pub fn eu075() -> String {
    fn pyth(n: usize) -> Vec<usize> {
        fn p(n: usize, a: usize, b: usize, c: usize) -> Vec<usize> {
            if a + b + c >= n {
                return vec![];
            }
            let v = a + b + c;
            let mut res: Vec<usize> = Vec::new();
            res.push(v);
            res.append(&mut p(n,
                              c + c + a - b - b,
                              a + a + c + c - b,
                              a + a + c + c + c - b - b));
            res.append(&mut p(n,
                              a + b + b + c + c,
                              a + a + b + c + c,
                              a + a + b + b + c + c + c));
            res.append(&mut p(n,
                              c + c + b + b - a,
                              c + c + b - a - a,
                              c + c + c + b + b - a - a));

            res.sort();
            res
        }
        p(n, 3, 4, 5)
    }

    let n: usize = 1_500_000;
    let list = pyth(n);
    let mut table: Vec<usize> = vec![0; n+1 as usize];
    for v in &list {
        table[*v] += 1;
    }
    for v in &list {
        for i in 2.. {
            let new = i * v;
            if i * v > n {
                break;
            }
            table[new] += 1;
        }
    }

    let res = table.into_iter().fold(0, |acc, x| if x == 1 { acc + x } else { acc });
    assert_eq!(res, 161667);
    format!("eu075 = {}", res)
} // 161667

/// Counting summations
pub fn eu076() -> String {
    fn solve(n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        let mut sum = vec![0; n+1];
        sum[0] = 1;
        for i in 1..n {
            for (j, _) in sum.clone().iter().enumerate().take(n + 1).skip(i) {
                sum[j] += sum[j - i];
            }
        }
        sum[n]
    }
    assert_eq!(solve(5), 6);

    let res = solve(100);
    assert_eq!(res, 190569291);
    format!("eu076 = {:?}", res)
} // 190569291

/// Prime summations
pub fn eu077() -> String {
    fn prime_sumation() -> usize {
        let mut ps: Vec<usize> = vec![0; 1001 as usize];
        for (i, _) in ps.clone().iter().enumerate().take(1000).skip(2) {
            let mut sum = 0;
            for j in 1..i {
                sum += primes::sopf(j) * ps[i - j]
            }
            ps[i] = (primes::sopf(i) + sum) / i;
            if ps[i] > 5000 {
                return i;
            }
        }
        0
    }

    let res = prime_sumation();
    assert_eq!(res, 71);
    format!("eu077 = {}", res)
} // 71

/// Coin partitions
pub fn eu078() -> String {
    // custom version of euler_library::integer_partitions()
    fn partitions() -> usize {
        // list of pentagonal numbers
        let k = (1..250)
                    .flat_map(|i| vec![i * (3 * i - 1) / 2, i * (3 * i - 1) / 2 + i])
                    .collect::<Vec<_>>();

        let mut ps: Vec<i64> = vec![1];
        let sign: Vec<i64> = vec![1, 1, -1, -1];
        let mut n = 0;
        while ps[n] != 0 {
            n += 1;
            let mut t: i64 = 0;
            let mut i = 0;
            while k[i] <= n {
                t += (ps[n - k[i]] as i64) * sign[i % 4];
                i += 1;
            }
            ps.push(t % 1_000_000)
        }
        n
    }

    let res = partitions();
    assert_eq!(res, 55374);
    format!("eu078 = {}", res)
} // 55374

/// Passcode derivation
pub fn eu079() -> String {
    let mut xs = vec![319, 680, 180, 690, 129, 620, 762, 689, 762, 318, 368, 710, 720, 710, 629, 168, 160, 689, 716,
                      731, 736, 729, 316, 729, 729, 710, 769, 290, 719, 680, 318, 389, 162, 289, 162, 718, 729, 319,
                      790, 680, 890, 362, 319, 760, 316, 729, 380, 319, 728, 716];

    xs.sort();
    let xss = xs.into_iter().map(eu::to_bytes).collect::<Vec<_>>();

    fn get_next(mut val: u8, xss: Vec<Vec<u8>>, set: Vec<u8>) -> u8 {
        for j in 1..3 {
            for xs in &xss {
                if xs[j] == val {
                    let t = get_next(xs[j - 1], xss.clone(), set.clone());
                    if set.contains(&t) {
                        break;
                    }
                    val = t;
                    break;
                }
            }
        }
        val
    }

    let mut val = xss[0][0];
    let mut set: Vec<u8> = Vec::new();
    for _ in 1..9 {
        val = get_next(val, xss.clone(), set.clone());
        set.push(val);
        'outer: for j in 0..2 {
            for xs in &xss {
                if xs[j] == val {
                    val = xs[j + 1];
                    break 'outer;
                }
            }
        }
    }

    let res = eu::from_bytes::<usize>(&set).unwrap();
    assert_eq!(res, 73162890);
    format!("eu079 = {:?}", res)
} // 73162890

/// Square root digital expansion
pub fn eu080() -> String {
    let perfect = vec![4, 9, 16, 25, 36, 49, 64, 81, 100];
    let sum = (2..101)
                  .filter(|x| !perfect.contains(x))
                  .fold(0, |acc, x| {
                      let sqrt_str = eu_big::precision_sqrt(x, 100).to_string();
                      acc + eu::sum_of_digits(sqrt_str)
                  });

    assert_eq!(sum, 40886);
    format!("eu080 = {}", sum)
} // 40886

/// Returns Vec of the Euler solution functions in this crate.
pub fn get_functions() -> Vec<fn() -> String> {
    vec![eu071, eu072, eu073, eu074, eu075, eu076, eu077, eu078, eu079, eu080]
}
