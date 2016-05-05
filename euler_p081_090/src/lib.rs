//! Project Euler solutions for problems 81 through 90.
//!
//! This crate is designed to be used via crate `euler`.
use std::cmp;
use std::f64::EPSILON;

extern crate primal;

extern crate itertools;
use itertools::Itertools;

extern crate rand;
use rand::{Rng, thread_rng};

extern crate numerals;
use numerals::roman::Roman;

extern crate petgraph;
use petgraph::*;
use petgraph::algo::dijkstra;

/// Returns data from matrix.txt. Used by p081, p082 and p083
pub fn get_data() -> Vec<usize> {
    let buffer = include_str!("../data/matrix.txt");
    buffer.lines()
          .flat_map(|x| {
              x.split(',')
               .map(|x| x.parse().unwrap())
               .collect::<Vec<usize>>()
          })
          .collect()
}

/// Path sum: two ways
pub fn p081() -> String {
    fn get_edges(i: u32, ws: &[usize], cols: u32) -> Vec<(u32, u32, usize)> {
        // first node
        if i == 0 {
            return vec![((i, i + 1, ws[i as usize]))];
        }
        let mut edges = Vec::new();
        // right if not last col
        if i % cols != 0 {
            edges.push((i, i + 1, ws[i as usize]))
        }
        // down if not last row
        if i + cols <= ws.len() as u32 {
            edges.push((i, i + cols, ws[(i + cols - 1) as usize]))
        }
        edges
    }

    fn make_graph(weights: Vec<usize>) -> Graph<usize, usize> {
        let mut g = Graph::<usize, usize>::new();
        for i in 0..weights.len() + 1 {
            g.add_node(i);
        }
        let cols = (weights.len() as f64).sqrt() as u32;
        let edges = (0..weights.len())
                        .flat_map(|i| get_edges(i as u32, &weights, cols))
                        .collect::<Vec<_>>();

        g.extend_with_edges(&edges);
        g
    }

    fn solve(g: Graph<usize, usize>, idxs: Vec<graph::NodeIndex>) -> usize {
        let start_node = idxs[0];
        let finish_node = idxs.last().unwrap();
        let sol = dijkstra(&g, // directed graph
                           start_node, // start node
                           Some(*finish_node), // finishing node
                           |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));  // edges for each node iterator

        *sol.get(finish_node).unwrap()
    }

    // test
    let xss = vec![vec![131, 673, 234, 103, 18],
                   vec![201, 96, 342, 965, 150],
                   vec![630, 803, 746, 422, 111],
                   vec![537, 699, 497, 121, 956],
                   vec![805, 732, 524, 37, 331]];
    let xs = xss.into_iter().flat_map(|x| x).collect();
    let g = make_graph(xs);
    let idxs = g.node_indices().collect::<Vec<_>>();
    let res = solve(g, idxs);
    assert_eq!(res, 2427);

    let g = make_graph(get_data());
    let idxs = g.node_indices().collect::<Vec<_>>();
    let res = solve(g, idxs);

    assert_eq!(res, 427337);
    format!("p081 = {}", res)
} // 427337

/// Path sum: three ways
pub fn p082() -> String {
    fn get_edges(i: u32, ws: &[usize], cols: u32) -> Vec<(u32, u32, usize)> {
        // first nodes (whole first column)
        if i == 0 {
            return (0..ws.len())
                       .step(cols as usize)
                       .map(|idx| (0 as u32, (idx + 1) as u32, ws[idx]))
                       .collect_vec();
        }
        let mut edges = Vec::new();
        // right if not last col
        if i % cols != 0 {
            edges.push((i, i + 1, ws[i as usize]))
        }
        // down if not last row
        if i + cols <= ws.len() as u32 {
            edges.push((i, i + cols, ws[(i + cols - 1) as usize]))
        }
        // up if not first row
        if i > cols {
            edges.push((i, i - cols, ws[(i - cols - 1) as usize]))
        }
        // up for last element
        if i + 1 == ws.len() as u32 {
            edges.push((i + 1, i + 1 - cols, ws[(i + 1 - cols - 1) as usize]))
        }
        edges
    }

    fn make_graph(weights: Vec<usize>) -> Graph<usize, usize> {
        let mut g = Graph::<usize, usize>::new();
        for i in 0..weights.len() + 1 {
            g.add_node(i);
        }
        let cols = (weights.len() as f64).sqrt() as u32;
        let edges = (0..weights.len())
                        .flat_map(|i| get_edges(i as u32, &weights, cols))
                        .collect::<Vec<_>>();

        g.extend_with_edges(&edges);
        g
    }

    fn solve(g: Graph<usize, usize>, idxs: Vec<graph::NodeIndex>) -> (usize, usize) {
        let start_node = idxs[0];
        let solution = dijkstra(&g, // directed graph
                                start_node, // start node
                                None, // finishing node
                                |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));

        let col = ((solution.len() - 1) as f64).sqrt() as usize;
        let sol_vec = solution.into_iter()
                              .filter_map(|(n, s)| if g[n] % col == 0 && s != 0 { Some((s, g[n])) } else { None })
                              .sorted();
        sol_vec.into_iter().min().unwrap()
    }

    // test
    let xss = vec![vec![131, 673, 234, 103, 18],
                   vec![201, 96, 342, 965, 150],
                   vec![630, 803, 746, 422, 111],
                   vec![537, 699, 497, 121, 956],
                   vec![805, 732, 524, 37, 331]];
    let xs = xss.into_iter().flat_map(|x| x).collect();
    let g = make_graph(xs);
    let idxs = g.node_indices().collect::<Vec<_>>();
    let (res, _) = solve(g, idxs);
    assert_eq!(res, 994);

    let g = make_graph(get_data());
    let idxs = g.node_indices().collect::<Vec<_>>();
    let (res, _) = solve(g, idxs);
    assert_eq!(res, 260324);
    format!("p082 = {}", res)
} // 260324

/// Path sum: four ways
pub fn p083() -> String {
    fn get_edges(i: u32, ws: &[usize], cols: u32) -> Vec<(u32, u32, usize)> {
        // first node
        if i == 0 {
            return vec![((i, i + 1, ws[i as usize]))];
        }
        let mut edges = Vec::new();
        // right if not last col
        if i % cols != 0 {
            edges.push((i, i + 1, ws[i as usize]))
        }
        // down if not last row
        if i + cols <= ws.len() as u32 {
            edges.push((i, i + cols, ws[(i + cols - 1) as usize]))
        }
        // up if not first row
        if i > cols {
            edges.push((i, i - cols, ws[(i - cols - 1) as usize]))
        }
        // left if not first col
        if (i % cols) != 1 {
            edges.push((i, i - 1, ws[(i - 2) as usize]));
        }
        edges
    }

    fn make_graph(weights: Vec<usize>) -> Graph<usize, usize> {
        let mut g = Graph::<usize, usize>::new();
        for i in 0..weights.len() + 1 {
            g.add_node(i);
        }
        let cols = (weights.len() as f64).sqrt() as u32;
        let edges = (0..weights.len())
                        .flat_map(|i| get_edges(i as u32, &weights, cols))
                        .collect::<Vec<_>>();

        g.extend_with_edges(&edges);
        g
    }

    fn solve(g: Graph<usize, usize>, idxs: Vec<graph::NodeIndex>) -> usize {
        let start_node = idxs[0];
        let finish_node = idxs.last().unwrap();
        let sol = dijkstra(&g, // directed graph
                           start_node, // start node
                           Some(*finish_node), // finishing node
                           |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));  // edges for each node iterator

        *sol.get(finish_node).unwrap()
    }

    // test
    let xss = vec![vec![131, 673, 234, 103, 18],
                   vec![201, 96, 342, 965, 150],
                   vec![630, 803, 746, 422, 111],
                   vec![537, 699, 497, 121, 956],
                   vec![805, 732, 524, 37, 331]];
    let xs = xss.into_iter().flat_map(|x| x).collect();
    let g = make_graph(xs);
    let idxs = g.node_indices().collect::<Vec<_>>();
    let res = solve(g, idxs);
    assert_eq!(res, 2297);

    let g = make_graph(get_data());
    let idxs = g.node_indices().collect::<Vec<_>>();
    let res = solve(g, idxs);
    assert_eq!(res, 425185);
    format!("p083 = {}", res)
} // 425185

/// Monopoly odds
pub fn p084() -> String {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    enum B { GO = 0, A1, CC1, A2, T1, R1, B1, CH1, B2, B3, JAIL, C1, U1, C2, C3, R2, D1, CC2, D2,
             D3, FP, E1, CH2, E2, E3, R3, F1, F2, U2, F3, G2J, G1, G2, CC3, G3, R4, CH3, H1, T2, H2,}

    #[derive(Debug)]
    struct Sim {
        name: B,
        visits: u32,
    }

    #[derive(Debug)]
    struct Pos {
        pos: usize,
        chance_pos: usize,
        cc_pos: usize,
        double_cnt: usize,
    }

    let mut game: &mut Vec<Sim> = &mut vec![Sim { name: B::GO, visits: 0 },
                                            Sim { name: B::A1, visits: 0 },
                                            Sim { name: B::CC1, visits: 0 },
                                            Sim { name: B::A2, visits: 0 },
                                            Sim { name: B::T1, visits: 0 },
                                            Sim { name: B::R1, visits: 0 },
                                            Sim { name: B::B1, visits: 0 },
                                            Sim { name: B::CH1, visits: 0 },
                                            Sim { name: B::B2, visits: 0 },
                                            Sim { name: B::B3, visits: 0 },
                                            Sim { name: B::JAIL, visits: 0 },
                                            Sim { name: B::C1, visits: 0 },
                                            Sim { name: B::U1, visits: 0 },
                                            Sim { name: B::C2, visits: 0 },
                                            Sim { name: B::C3, visits: 0 },
                                            Sim { name: B::R2, visits: 0 },
                                            Sim { name: B::D1, visits: 0 },
                                            Sim { name: B::CC2, visits: 0 },
                                            Sim { name: B::D2, visits: 0 },
                                            Sim { name: B::D3, visits: 0 },
                                            Sim { name: B::FP, visits: 0 },
                                            Sim { name: B::E1, visits: 0 },
                                            Sim { name: B::CH2, visits: 0 },
                                            Sim { name: B::E2, visits: 0 },
                                            Sim { name: B::E3, visits: 0 },
                                            Sim { name: B::R3, visits: 0 },
                                            Sim { name: B::F1, visits: 0 },
                                            Sim { name: B::F2, visits: 0 },
                                            Sim { name: B::U2, visits: 0 },
                                            Sim { name: B::F3, visits: 0 },
                                            Sim { name: B::G2J, visits: 0 },
                                            Sim { name: B::G1, visits: 0 },
                                            Sim { name: B::G2, visits: 0 },
                                            Sim { name: B::CC3, visits: 0 },
                                            Sim { name: B::G3, visits: 0 },
                                            Sim { name: B::R4, visits: 0 },
                                            Sim { name: B::CH3, visits: 0 },
                                            Sim { name: B::H1, visits: 0 },
                                            Sim { name: B::T2, visits: 0 },
                                            Sim { name: B::H2, visits: 0 }];

    // fn print_board(xs: &mut Vec<Sim>) {
    //     for (i, x) in xs.iter().enumerate() {
    //         println!("Pos: {:2} ({:?}), visits = {:?}", i, x.name, x.visits);
    //     }
    // }

    fn roll_two_dice(sides: usize) -> (usize, usize) {
        let mut rng = thread_rng();
        let d1: usize = rng.gen_range(1, sides + 1);
        let d2: usize = rng.gen_range(1, sides + 1);
        (d1, d2)
    }

    fn find_ranks(game: &[Sim]) -> Vec<(usize, usize)> {
        let temp = game.iter().map(|x| x.visits as f64).zip(0..).collect::<Vec<_>>();
        let sum = temp.iter().fold(0.0, |acc, x| acc + x.0);
        // println!("sum = {:?}", sum);
        let mut res = temp.iter()
                          .map(|&(x, i)| ((x / sum * 100000.0) as usize, i))
                          .collect::<Vec<(usize, usize)>>();
        res.sort();
        res.reverse();
        res
    }

    fn chance(mut pos: Pos, game: &mut Vec<Sim>) -> Pos {
        let chance = vec![B::GO, B::JAIL, B::C1, B::E3, B::H2, B::R1];
        pos.chance_pos = (pos.chance_pos + 1) % 16;
        if pos.chance_pos < 6 {
            if pos.chance_pos == 2 {
                pos = cc(pos);
            } else {
                pos.pos = chance[pos.chance_pos] as usize;
            }
            if chance[pos.chance_pos] == B::JAIL {
                pos.double_cnt = 0;
            }
        } else {
            match pos.chance_pos {
                6 | 7 => {
                    match game[pos.pos].name {
                        B::CH1 => pos.pos = B::R2 as usize,
                        B::CH2 => pos.pos = B::R3 as usize,
                        B::CH3 => pos.pos = B::R1 as usize,
                        _ => pos.pos = pos.pos,
                    }
                }
                8 => {
                    match game[pos.pos].name {
                        B::CH1 | B::CH3 => pos.pos = B::U1 as usize,
                        B::CH2 => pos.pos = B::U2 as usize,
                        _ => pos.pos = pos.pos,
                    }
                }
                9 => pos.pos = pos.pos - 3,
                _ => pos.pos = pos.pos,
            }
        }
        pos
    }

    fn cc(mut pos: Pos) -> Pos {
        let cc = vec![B::GO, B::JAIL];
        pos.cc_pos = (pos.cc_pos + 1) % 16;
        if pos.cc_pos < 2 {
            pos.pos = cc[pos.cc_pos] as usize;
        }
        pos
    }

    fn do_move(mut pos: Pos, game: &mut Vec<Sim>) -> Pos {
        let (d1, d2) = roll_two_dice(SIDES);
        if d1 == d2 {
            if pos.double_cnt == 2 {
                pos.double_cnt = 0;
                pos.pos = B::JAIL as usize;
                game[B::JAIL as usize].visits += 1;
                return pos;
            } else {
                pos.double_cnt += 1
            }
        } else {
            pos.double_cnt = 0
        }
        pos.pos = (pos.pos + d1 + d2) % 40;
        match game[pos.pos].name {
            B::CH1 | B::CH2 | B::CH3 => pos = chance(pos, game),
            B::CC1 | B::CC2 | B::CC3 => pos = cc(pos),
            B::G2J => {
                pos.double_cnt = 0;
                pos.pos = B::JAIL as usize;
            }
            _ => pos.pos = pos.pos,
        }
        game[pos.pos].visits += 1;
        pos
    }

    const SIDES: usize = 4;

    let mut pos: Pos = Pos { pos: 0, chance_pos: 0, cc_pos: 0, double_cnt: 0 };
    for _ in 0..500_000 {
        pos = do_move(pos, game);
    }
    let ranks = find_ranks(&game);
    let most = ranks.iter()
                    .take(3)
                    .map(|&(_, i)| if i == 0 { "00".to_string() } else { i.to_string() })
                    .collect::<Vec<String>>();
    // print_board(game);

    let res = most.join("").parse::<usize>().unwrap();
    assert_eq!(res, 101524);
    format!("p084 = {}", most.join(""))
} // 101524 jail:r2:e3

/// Counting rectangles
pub fn p085() -> String {
    const MAX: i64 = 2_000_000;
    let mut diff = MAX;
    let mut area = 0;
    // (x,y) -> (53,54) and (2,2000) yields n > MAX
    for x in 2..53 {
        for y in x..2000 {
            let n = x * (x + 1) * y * (y + 1) / 4;
            let new_diff = (MAX - n).abs();
            if new_diff < diff {
                diff = new_diff;
                area = x * y;
            }
        }
    }
    assert_eq!(area, 2772);
    format!("p085 = {}", area)
} // 2772

/// Cuboid route
pub fn p086() -> String {
    let mut cnt = 0.0;
    let mut j: f64 = 2.0;
    while cnt <= 1_000_000.0 {
        j += 1.0;
        for i in 3..2 * j as usize {
            let (ab, c) = (i as f64, j as f64);
            let sqrt = ((ab).powf(2.0) + c * c).sqrt();
            if (sqrt - sqrt.floor()).abs() < EPSILON {
                if ab < j { cnt += ab / 2.0 } else { cnt += j + 1.0 - (ab + 1.0) / 2.0 }
            }
        }
    }

    assert_eq!(j as usize, 1818);
    format!("p086 = {}", j)
} // 1818

/// Prime power triples
pub fn p087() -> String {
    const MAX: usize = 50_000_000;
    let primes = primal::Primes::all()
                     .take_while(|&p| p < ((MAX as f64).sqrt() * 1.1) as usize)
                     .collect_vec();

    let res = primes.iter()
                    .take_while(|&x| x.pow(4) < MAX)
                    .flat_map(|a| {
                        primes.iter()
                              .take_while(|&x| x.pow(3) < MAX)
                              .flat_map(|b| {
                                  primes.iter()
                                        .map(|c| a.pow(4) + b.pow(3) + c.pow(2))
                                        .filter(|&x| x < MAX)
                                        .collect_vec()
                              })
                              .collect_vec()
                    })
                    .sorted()
                    .into_iter()
                    .dedup()
                    .fold(0, |acc, _| acc + 1);

    assert_eq!(res, 1097343);
    format!("p087 = {}", res)
} // 1097343

/// Product-sum numbers
pub fn p088() -> String {
    fn merge(a: usize, b: usize, mut cache: &mut Vec<Vec<Vec<usize>>>, mut res: &mut Vec<Vec<usize>>) {
        if !cache[a].is_empty() && cache[a][0][0] == 0 {
            cache[a] = factor_lists(a, &mut cache);
        }
        for v in cache[a].clone() {
            let mut temp = v.clone();
            temp.push(b);
            temp.sort();
            res.push(temp);
        }
    }

    fn factor_lists(n: usize, mut cache: &mut Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
        let mut res = vec![];
        let mut limit = n;
        let mut i = 2;
        while i < limit {
            if n % i == 0 {
                limit = n / i;
                res.push(vec![i, limit]);
                merge(i, limit, &mut cache, &mut res);
                merge(limit, i, &mut cache, &mut res);
            }
            i += 1;
        }
        res.sort();
        res.dedup();
        res
    }

    fn sum(xs: &[usize]) -> usize {
        xs.iter().fold(0, |acc, &x| acc + x)
    }

    const MAX: usize = 12000;

    let mut k: Vec<usize> = vec![MAX*2; MAX+1];
    let mut cache: Vec<Vec<Vec<usize>>> = vec![vec![vec![0]]; MAX];
    k[0] = 0;
    k[1] = 0;
    let max = (MAX as f64 * 1.2) as usize;
    for i in 2..max {
        let factors = factor_lists(i, &mut cache);
        for v in factors {
            let j = i - sum(&v) + v.len();
            if j <= MAX {
                k[j] = cmp::min(k[j], i);
            }
        }
    }

    k.sort();
    k.dedup();

    let res = sum(&k);
    assert_eq!(res, 7587457);
    format!("p088 = {}", sum(&k))
} // 7587457

/// Roman numerals
pub fn p089() -> String {
    let romans_in = include_str!("../data/p089_roman.txt")
                        .lines()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();

    let char_count_in = romans_in.iter()
                                 .map(|x| x.len())
                                 .fold(0, |acc, x| acc + x);

    let numbers = romans_in.iter()
                           .map(|x| Roman::parse(x).unwrap().value())
                           .collect::<Vec<_>>();

    let romans_out = numbers.iter()
                            .map(|&x| format!("{:X}", Roman::from(x)))
                            .collect::<Vec<_>>();

    let char_count_out = romans_out.iter()
                                   .map(|x| x.len())
                                   .fold(0, |acc, x| acc + x);

    assert_eq!(char_count_in - char_count_out, 743);
    format!("p089 = {}", char_count_in - char_count_out)
} // 743

/// Cube digit pairs
pub fn p090() -> String {
    let sqrs = vec![(0, 1), (0, 4), (0, 6), (1, 6), (2, 5), (3, 6), (4, 6), (8, 1)];

    let is_valid = |xs: &Vec<usize>, ys: &Vec<usize>| -> bool {
        sqrs.iter()
            .all(|&(a, b)| (xs.contains(&a) && ys.contains(&b)) || (xs.contains(&b) && ys.contains(&a)))
    };

    let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 6];
    let cube = list.into_iter().combinations_n(6).collect_vec();

    let cnt = cube.iter()
                  .enumerate()
                  .flat_map(|(i, xs)| {
                      cube.iter()
                          .enumerate()
                          .take_while(|&(j, _)| j < i)
                          .map(|(_, ys)| is_valid(xs, ys))
                          .collect_vec()
                  })
                  .fold(0, |acc, x| if x { acc + 1 } else { acc });

    assert_eq!(cnt, 1217);
    format!("p090 = {}", cnt)
} // 1217

/// Returns (start, Vec of solution functions) for all solutions in this crate.
pub fn get_functions() -> (u32, Vec<fn() -> String>) {
    // Euler solutions in this crate.
    (81,
     vec![p081, p082, p083, p084, p085, p086, p087, p088, p089, p090])
}
