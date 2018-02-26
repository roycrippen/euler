//! Project Euler solutions for problems 101 through 110.
//!
//! This crate is designed to be used via crate `euler`.

use std::mem;

extern crate itertools;
use itertools::Itertools;

extern crate petgraph;
use petgraph::Graph;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;

extern crate euler_library;
use euler_library::common as eu;
use euler_library::primes;

/// Optimum polynomial
pub fn p101() -> String {
  fn un(n: usize) -> f64 {
    let nf = n as f64;
    1.0 - nf + nf.powf(2.0) - nf.powf(3.0) + nf.powf(4.0) - nf.powf(5.0) + nf.powf(6.0) - nf.powf(7.0) + nf.powf(8.0)
      - nf.powf(9.0) + nf.powf(10.0)
  }

  fn get_lhs(n: usize) -> Vec<Vec<f64>> {
    let mut res = Vec::new();
    for i in 1..n + 1 {
      let mut t = Vec::new();
      for j in 0..n {
        t.push(i.pow(j as u32) as f64);
      }
      t.reverse();
      // t.push(rhs[i - 1]);
      res.push(t);
    }
    res
  }

  fn get_bop(xs: &mut Vec<f64>) -> f64 {
    let l = xs.len();
    xs.reverse();
    (0..l).fold(0.0, |acc, i| {
      let pows = ((l + 1).pow(i as u32)) as f64;
      acc + xs[i] * pows
    })
  }

  fn check(xs: &[f64]) -> f64 {
    let l = xs.len();
    let mut ys = xs.to_vec();
    ys.reverse();
    (0..l).fold(0.0, |acc, i| {
      let pows = (l.pow(i as u32)) as f64;
      acc + ys[i] * pows
    })
  }

  // from http://introcs.cs.princeton.edu/java/95linear/GaussianElimination.java.html
  // Gaussian elimination with partial pivoting
  fn solve_la(lhs: &[Vec<f64>], rhs: &[f64]) -> Vec<f64> {
    let a = &mut lhs.to_vec();
    let b = &mut rhs.to_vec();
    let n = b.len();
    for p in 0..n {
      // find pivot row and swap
      let mut max = p;
      for (i, _) in a.iter().enumerate().take(n).skip(p + 1) {
        if a[i][p].abs() > a[max][p].abs() {
          max = i;
        }
      }
      let temp = a[p].clone();
      a[p] = a[max].clone();
      a[max] = temp;
      b.swap(p, max);

      // singular or nearly singular
      if a[p][p].abs() <= EPSILON {
        panic!("Matrix is singular or nearly singular");
      }

      // pivot within a and b
      for i in p + 1..n {
        let alpha = a[i][p] / a[p][p];
        b[i] -= alpha * b[p];
        for j in p..n {
          a[i][j] -= alpha * a[p][j];
        }
      }
    }

    // back substitution
    let mut x = vec![0.0 as f64; n];
    let mut idx = n - 1;
    loop {
      let mut sum = 0.0;
      for (j, _) in x.iter().enumerate().take(n).skip(idx + 1) {
        sum += a[idx][j] * x[j];
      }
      x[idx] = ((b[idx] - sum) / a[idx][idx]).round();
      if idx == 0 {
        break;
      }
      idx -= 1;
    }
    x
  }

  static EPSILON: f64 = 1.0e-10;

  let full_rhs = (1..11).map(un).collect_vec();

  let mut result = 0.0;
  for i in 1..11 {
    let lhs = get_lhs(i);
    let rhs = full_rhs.iter().take(i).cloned().collect_vec();
    let mut x = solve_la(&lhs, &rhs);
    if i == 2 {
      assert_eq!(un(2) as usize, check(&x) as usize)
    }
    result += get_bop(&mut x);
  }

  assert_eq!(result as usize, 37076114526);
  format!("p101 = {}", result)
} // 37076114526

/// Triangle containment
pub fn p102() -> String {
  #[derive(Debug, Clone, Copy)]
  struct P {
    x: i32,
    y: i32,
  };

  // get the triangle coordinates file
  fn get_points() -> Vec<Vec<P>> {
    let buffer = include_str!("../data/p102_triangles.txt");
    let xs = buffer
      .lines()
      .map(|x| {
        x.split(',')
          .map(|x| x.parse().unwrap())
          .collect::<Vec<i32>>()
      })
      .collect_vec();
    xs.into_iter()
      .map(|z| {
        vec![
          P { x: z[0], y: z[1] },
          P { x: z[2], y: z[3] },
          P { x: z[4], y: z[5] },
        ]
      })
      .collect_vec()
  }

  // https://en.wikipedia.org/wiki/Barycentric_coordinate_system
  fn alpha(p1: P, p2: P, p3: P, p: P) -> f64 {
    ((p2.y - p3.y) * (p.x - p3.x) + (p3.x - p2.x) * (p.y - p3.y)) as f64
      / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y)) as f64
  };

  fn beta(p1: P, p2: P, p3: P, p: P) -> f64 {
    ((p3.y - p1.y) * (p.x - p3.x) + (p1.x - p3.x) * (p.y - p3.y)) as f64
      / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y)) as f64
  };

  fn gamma(alpha: f64, beta: f64) -> f64 {
    1.0 - alpha - beta
  };

  fn is_contained(p1: P, p2: P, p3: P) -> bool {
    let origin = P { x: 0, y: 0 };
    let alpha_ = alpha(p1, p2, p3, origin);
    let beta_ = beta(p1, p2, p3, origin);
    let gamma_ = gamma(alpha_, beta_);
    alpha_ > 0.0 && beta_ > 0.0 && gamma_ > 0.0
  }

  // contained
  let a = P { x: -340, y: 495 };
  let b = P { x: -153, y: -910 };
  let c = P { x: 835, y: -947 };
  assert!(is_contained(a, b, c));

  // not contained
  let a = P { x: -175, y: 41 };
  let b = P { x: -421, y: -714 };
  let c = P { x: 574, y: -645 };
  assert!(!is_contained(a, b, c));

  let res = get_points().into_iter().fold(0, |acc, xs| {
    if is_contained(xs[0], xs[1], xs[2]) {
      acc + 1
    } else {
      acc
    }
  });

  assert_eq!(res, 228);
  format!("p102 = {}", res)
} // 228

/// Special subset sums: optimum
pub fn p103() -> String {
  fn set_string(set: &[usize]) -> String {
    set.iter().fold("".to_string(), |acc, x| {
      acc + &x.to_string()
    })
  }

  fn vec_sum(set: &[usize]) -> usize {
    set.into_iter().fold(0, |acc, x| acc + x)
  }

  fn has_duplicates<T: Ord>(xs: &[T]) -> bool {
    let l = xs.into_iter().dedup().collect_vec().len();
    l != xs.len()
  }

  // if xs = [1,2,3,4,5,6,7] check that
  // sum([1,2]) > sum([7]) && sum([1,2,3]>sum([6,7]) && sum([1,2,3,4] > sum([5,6,7])))
  // for all count of { sum(lhs) <= sum(rhs) } == 0
  fn pass_rule_2(xs: &[usize]) -> bool {
    if xs.len() < 3 {
      return true;
    }
    let t = eu::accumulate(xs);
    let lhs = t.iter().skip(1).take(xs.len() / 2).collect_vec();

    let t = eu::accumulate(&xs.iter().rev().cloned().collect_vec());
    let rhs = t.iter().take(xs.len() / 2).collect_vec();

    lhs
      .iter()
      .zip(rhs.iter())
      .filter(|&(&x, &y)| x <= y)
      .count() == 0
  }

  fn pass_rule_1(xs: &[usize]) -> bool {
    for i in 3..(xs.len() / 2) + 1 {
      let sums = xs.iter()
        .combinations(i)
        .into_iter()
        .map(|x| x.iter().fold(0, |acc, &&y| acc + y))
        .sorted();
      if has_duplicates(&sums) {
        return false;
      }
    }
    true
  }

  fn is_sss(a: &[usize]) -> bool {
    if has_duplicates(a) || !pass_rule_2(a) || !pass_rule_1(a) {
      return false;
    }
    true
  }

  fn make_candidates(vec: &[usize]) -> Vec<Vec<usize>> {
    let adjust = &vec![-2, -1, 1, 2];
    let mut candidates: Vec<Vec<usize>> = Vec::new();
    for i in 0..vec.len() {
      for j in i..vec.len() {
        for v in adjust {
          let mut t = vec.to_vec();
          t[j] = (t[i] as i32 + v) as usize;
          t = t.iter().sorted().iter().dedup().map(|&&x| x).collect_vec();
          candidates.push(t)
        }
      }
    }
    candidates = candidates
      .into_iter()
      .sorted()
      .into_iter()
      .dedup()
      .filter(|x| x.len() == vec.len())
      .collect_vec();
    candidates
  }

  fn solve(start: &[usize]) -> String {
    let mut best = start.to_vec();
    let mut min = vec_sum(&best);
    let candidates = make_candidates(&best);
    for candidate in candidates {
      if is_sss(&candidate) {
        let sum = vec_sum(&candidate);
        if sum < min {
          best = candidate.clone();
          min = sum
        }
      }
    }
    set_string(&best)
  }

  let res = solve(&[20, 31, 38, 39, 40, 42, 45]);
  assert_eq!(res, "20313839404245".to_string());
  format!("p103 = {}", res)
} // 20313839404245

/// Pandigital Fibonacci ends
pub fn p104() -> String {
  let big = (10 as usize).pow(9);
  let root5 = (5.0 as f64).sqrt();
  let phi = (1.0 + root5) / 2.0;

  let mut a: usize = 0;
  let mut b: usize = 1;
  let mut cnt = 1;
  loop {
    a += b;
    // let t = a;
    // a = b;
    // b = t;
    mem::swap(&mut a, &mut b);
    a %= big;
    b %= big;
    cnt += 1;
    let back = b.to_string();
    if eu::is_pandigital(back, 1) {
      // https://en.wikipedia.org/wiki/Fibonacci_number
      let logfib = (cnt as f64) * phi.log(10.0) - root5.log(10.0);
      let str = ((10.0 as f64).powf(logfib - logfib.floor() + 8.0) as usize).to_string();
      if eu::is_pandigital(str, 1) {
        break;
      }
    }
  }

  assert_eq!(cnt, 329468);
  format!("p104 = {}", cnt)
} // 329468

/// Special subset sums: testing
pub fn p105() -> String {
  fn get_data() -> Vec<Vec<usize>> {
    let buffer = include_str!("../data/p105_sets.txt");
    buffer
      .lines()
      .map(|x| x.split(',').map(|x| x.parse().unwrap()).sorted())
      .collect_vec()
  }

  fn vec_sum(set: &[usize]) -> usize {
    set.into_iter().fold(0, |acc, x| acc + x)
  }

  fn has_duplicates<T: Ord>(xs: &[T]) -> bool {
    let l = xs.into_iter().dedup().collect_vec().len();
    l != xs.len()
  }

  // if xs = [1,2,3,4,5,6,7] check that
  // sum([1,2]) > sum([7]) && sum([1,2,3]>sum([6,7]) && sum([1,2,3,4] > sum([5,6,7])))
  // for all count of { sum(lhs) <= sum(rhs) } == 0
  fn pass_rule_2(xs: &[usize]) -> bool {
    if xs.len() < 3 {
      return true;
    }
    let t = eu::accumulate(xs);
    let lhs = t.iter().skip(1).take(xs.len() / 2).collect_vec();

    let t = eu::accumulate(&xs.iter().rev().cloned().collect_vec());
    let rhs = t.iter().take(xs.len() / 2).collect_vec();

    lhs
      .iter()
      .zip(rhs.iter())
      .filter(|&(&x, &y)| x <= y)
      .count() == 0
  }

  fn pass_rule_1(xs: &[usize]) -> bool {
    for i in 3..(xs.len() / 2) + 1 {
      let sums = xs.iter()
        .combinations(i)
        .into_iter()
        .map(|x| x.iter().fold(0, |acc, &&y| acc + y))
        .sorted();
      if has_duplicates(&sums) {
        return false;
      }
    }
    true
  }

  fn is_sss(a: &[usize]) -> bool {
    if has_duplicates(a) || !pass_rule_2(a) || !pass_rule_1(a) {
      return false;
    }
    true
  }

  let sum = get_data()
    .into_iter()
    .fold(0, |acc, x| if is_sss(&x) { acc + vec_sum(&x) } else { acc });

  assert_eq!(sum, 73702);
  format!("p105 = {}", sum)
} // 73702

/// Special subset sums: meta-testing
pub fn p106() -> String {
  fn has_duplicates(xs: &[&usize], ys: &[&usize]) -> bool {
    let cnt = xs.iter().take_while(|x| !ys.contains(x)).count();
    cnt < xs.len()
  }

  let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

  let xs = (2..vec.len() / 2 + 1)
    .map(|i| vec.iter().combinations(i).sorted())
    .flatten()
    .sorted();

  let mut res = 0;
  for (i, v1) in xs.clone().iter().enumerate() {
    for v2 in xs.iter().skip(i) {
      if v1.len() + v2.len() > vec.len() || v1.len() != v2.len() {
        continue;
      }
      if !has_duplicates(v1, v2) {
        let mut at_least_one = false;
        for k in 0..v1.len() {
          if v1[k] > v2[k] {
            at_least_one = true;
            break;
          }
        }
        if at_least_one {
          res += 1
        }
      }
    }
  }

  assert_eq!(res, 21384);
  format!("p106 = {}", res)
} // 21384

/// Minimal network
pub fn p107() -> String {
  fn get_data() -> Vec<Vec<u32>> {
    let buffer = include_str!("../data/p107_network.txt");
    buffer
      .lines()
      .map(|x| {
        x.split(',')
          .map(|x| if x != "-" { x.parse().unwrap() } else { 0 })
          .collect::<Vec<u32>>()
      })
      .collect()
  }

  fn make_graph(xs: &[Vec<u32>]) -> Graph<(), u32> {
    let vec = xs.iter()
      .enumerate()
      .flat_map(|(i, v)| {
        (i..v.len())
          .map(|j| (i as u32, j as u32, v[j]))
          .collect::<Vec<_>>()
      })
      .filter(|&(_, _, v)| v != 0)
      .collect::<Vec<_>>();
    Graph::<(), u32>::from_edges(&vec)
  }

  let mut gr = make_graph(&get_data());
  let gr_sum: u32 = gr.edge_weights_mut().fold(0, |acc, &mut x| acc + x);
  let mut gr_res: Graph<(), u32> = FromElements::from_elements(min_spanning_tree(&gr));
  let gr_res_sum: u32 = gr_res.edge_weights_mut().fold(0, |acc, &mut x| acc + x);

  let res = gr_sum - gr_res_sum;
  assert_eq!(res, 259679);
  format!("p107 = {}", res)
} // 259679

/// Diophantine reciprocals I
pub fn p108() -> String {
  fn dio_recip_cnt(n: usize) -> usize {
    (primes::prime_factors(n)
      .into_iter()
      .group_by(|x| *x)
      .into_iter()
      .map(|(_, ys)| ys.into_iter().count())
      .fold(1, |acc, x| acc * (2 * x + 1)) + 1) / 2
  }

  fn solve() -> usize {
    let list = primes::prime_factor_cnt(300_000)
      .iter()
      .enumerate()
      .filter_map(|(i, &v)| if v > 5 { Some(i) } else { None })
      .collect_vec();

    for v in list {
      if dio_recip_cnt(v) > 1000 {
        return v;
      }
    }
    0
  }

  let res = solve();
  assert_eq!(res, 180180);
  format!("p108 = {}", res)
} // 180180

/// Darts
pub fn p109() -> String {
  fn get_scores() -> (Vec<usize>, Vec<usize>) {
    let mut scores = vec![0, 25, 50];
    let mut doubles = vec![50];
    for i in 1..21 {
      scores.push(i);
      scores.push(2 * i);
      scores.push(3 * i);
      doubles.push(2 * i);
    }
    (scores, doubles)
  }

  fn solve(n: usize) -> usize {
    let (all_scores, doubles) = get_scores();
    let mut cnt = 0;
    for (i, vi) in all_scores.iter().enumerate() {
      for vj in all_scores.iter().skip(i) {
        for vk in &doubles {
          if vi + vj + vk < n {
            cnt += 1;
          }
        }
      }
    }
    cnt
  }

  assert_eq!(solve(6), 11);

  let res = solve(100);
  assert_eq!(res, 38182);
  format!("p109 = {}", res)
} // 38182

/// Diophantine reciprocals II
pub fn p110() -> String {
  fn distinct_solution(xs: &[usize]) -> usize {
    (xs.into_iter().fold(1, |acc, x| acc * (2 * x + 1)) + 1) / 2
  }

  fn value(xs: &[usize]) -> usize {
    let primes = vec![
      2 as usize, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53
    ];
    xs.iter()
      .enumerate()
      .fold(1, |acc, (i, &v)| acc * primes[i].pow(v as u32) as usize)
  }

  fn get_candidates() -> Vec<Vec<usize>> {
    let n = 18;
    let xs = vec![2, 3, 4, 5];
    let mut candidates = Vec::new();
    for i in 1..7 {
      let mut combs = eu::k_nested_recur(i, &xs);
      for (j, v) in combs.clone().iter().enumerate() {
        let sum = v.iter().fold(0, |acc, x| acc + x);
        if sum > n {
          break;
        }
        combs[j].append(&mut vec![1; n - sum]);
      }
      candidates.append(&mut combs);
    }
    candidates
  }

  fn solve() -> usize {
    let solutions = get_candidates()
      .clone()
      .iter()
      .map(|x| (distinct_solution(x), x.clone()))
      .filter(|&(sol, _)| sol > 4_000_000)
      .sorted();

    let best_soltion = solutions
      .iter()
      .filter(|&&(sol, _)| sol == solutions[0].0)
      .map(|&(_, ref xs)| value(xs))
      .sorted();

    best_soltion[0]
  }

  let res = solve();
  assert_eq!(res, 9350130049860600);
  format!("p110 = {}", res)
} // 9350130049860600

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
  (
    101,
    vec![p101, p102, p103, p104, p105, p106, p107, p108, p109, p110],
  )
}
