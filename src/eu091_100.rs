//! Project Euler solutions for problems 91 through 100.

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use std::cmp;

extern crate num;
use self::num::integer::gcd;

extern crate itertools;
use self::itertools::Itertools;

extern crate euler_library;
use self::euler_library::common as eu;

/// Right triangles with integer coordinates
pub fn eu091() -> String {
    const SIZE: i32 = 50;

    let cnt = SIZE * SIZE * 3 +
              (1..SIZE + 1)
                  .flat_map(|x| {
                      (1..SIZE + 1)
                          .map(|y| {
                              let gcd = gcd(x, y);
                              let dx = x / gcd;
                              let dy = y / gcd;
                              cmp::min(y / dx, (SIZE - x) / dy) * 2
                          })
                          .collect::<Vec<_>>()
                  })
                  .fold(0, |acc, x| acc + x);

    assert_eq!(cnt, 14234);
    format!("eu091 = {}", cnt)
} // 14234

/// Square digit chains
pub fn eu092() -> String {

    const MAX: usize = 10_000_000;
    // sum_sq(9_999_999) = 567, all others are less
    const TABLE_SIZE: usize = 568;

    let sum_sq = |mut n: usize| -> usize {
        let mut total = 0;
        while n != 0 {
            total += (n % 10).pow(2);
            n /= 10
        }
        total
    };

    let mut table: Vec<usize> = vec![0; TABLE_SIZE];
    for i in 1..TABLE_SIZE {
        let mut val = i;
        while val != 1 && val != 89 {
            val = sum_sq(val)
        }
        table[i] = val
    }

    let cnt = (1..MAX).fold(0,
                            |acc, x| if table[sum_sq(x)] == 89 { acc + 1 } else { acc });

    assert_eq!(cnt, 8581146);
    format!("eu092 = {}", cnt)
} // 8581146

/// Arithmetic expressions
pub fn eu093() -> String {
    fn eval(a: f64, b: f64, op: char) -> f64 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            _ => if b != 0.0 { a / b } else { 100.0 },
        }
    }

    // x = (a op1 b) op2 (c op3 d)
    fn eval_group1(ns: &Vec<i32>, os: &Vec<char>, set: &mut HashSet<i32>) {
        let (a, b, c, d) = (ns[0] as f64, ns[1] as f64, ns[2] as f64, ns[3] as f64);
        let (op1, op2, op3) = (os[0], os[1], os[2]);
        let x = eval(eval(a, b, op1), eval(c, d, op3), op2);
        if x > 0.0 && x == x.floor() {
            set.insert(x as i32);
        }
    }

    // x = ((a op1 b) op2 c) op 3 d
    fn eval_group2(ns: &Vec<i32>, os: &Vec<char>, set: &mut HashSet<i32>) {
        let (a, b, c, d) = (ns[0] as f64, ns[1] as f64, ns[2] as f64, ns[3] as f64);
        let (op1, op2, op3) = (os[0], os[1], os[2]);
        let x = eval(eval(eval(a, b, op1), c, op2), d, op3);
        if x > 0.0 && x == x.floor() {
            set.insert(x as i32);
        }
    }

    // x = (a op1 (b op2 c)) op3 d
    fn eval_group3(ns: &Vec<i32>, os: &Vec<char>, set: &mut HashSet<i32>) {
        let (a, b, c, d) = (ns[0] as f64, ns[1] as f64, ns[2] as f64, ns[3] as f64);
        let (op1, op2, op3) = (os[0], os[1], os[2]);
        let x = eval(eval(a, eval(b, c, op2), op1), d, op3);
        if x > 0.0 && x == x.floor() {
            set.insert(x as i32);
        }
    }

    // x = a op1 (b op2 (c op3 d))
    fn eval_group4(ns: &Vec<i32>, os: &Vec<char>, set: &mut HashSet<i32>) {
        let (a, b, c, d) = (ns[0] as f64, ns[1] as f64, ns[2] as f64, ns[3] as f64);
        let (op1, op2, op3) = (os[0], os[1], os[2]);
        let x = eval(eval(b, eval(c, d, op3), op2), a, op1);
        if x > 0.0 && x == x.floor() {
            set.insert(x as i32);
        }
    }

    // x = a op1 ((b op2 c) op3 d))
    fn eval_group5(ns: &Vec<i32>, os: &Vec<char>, set: &mut HashSet<i32>) {
        let (a, b, c, d) = (ns[0] as f64, ns[1] as f64, ns[2] as f64, ns[3] as f64);
        let (op1, op2, op3) = (os[0], os[1], os[2]);
        let x = eval(eval(d, eval(b, c, op2), op3), a, op1);
        if x > 0.0 && x == x.floor() {
            set.insert(x as i32);
        }
    }

    fn count_set(set: HashSet<i32>) -> usize {
        set.into_iter()
           .sorted()
           .into_iter()
           .zip(1..)
           .filter(|&(a, b)| a == b)
           .count()
    }

    let mut max = (0, vec![]);
    let perm_ops = &eu::perms_with_reps(3, &vec!['+', '-', '*', '/']);
    let nums_comb = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().combinations_n(4);

    for nums in nums_comb {
        let mut set: HashSet<i32> = HashSet::new();
        let perm_nums = &eu::perms_without_reps_recur(4, &nums);
        for ns in perm_nums {
            for os in perm_ops {
                eval_group1(&ns, &os, &mut set);
                eval_group2(&ns, &os, &mut set);
                eval_group3(&ns, &os, &mut set);
                eval_group4(&ns, &os, &mut set);
                eval_group5(&ns, &os, &mut set);
            }
        }
        let cnt = count_set(set);
        if cnt > max.0 {
            max.0 = cnt;
            max.1 = nums.clone();
        }
    }
    max.1.sort();
    let s = format!("{}{}{}{}", max.1[0], max.1[1], max.1[2], max.1[3]);

    assert_eq!(s, "1258".to_string());
    format!("eu093 = {}", s)
} // 1258

/// Almost equilateral triangles
pub fn eu094() -> String {
    // v_short_side and v_long_side from:
    // http://www.had2know.com/academics/nearly-equilateral-heronian-triangles.html
    //
    // v_short_side(1) = 5; perimeter(1) = 5+5+6 = 3*5 + 1        println!("{:?}", p);
    fn v_short_side(n: usize) -> usize {
        match n {
            0 => 0,
            1 => 5,
            2 => 65,
            3 => 901,
            _ => 15 * v_short_side(n - 1) - 15 * v_short_side(n - 2) + v_short_side(n - 3),
        }
    }
    // v_long_side(1) = 16; perimeter(1) = 16+17+17 = 3*16 + 2
    fn v_long_side(n: usize) -> usize {
        match n {
            0 => 0,
            1 => 16,
            2 => 240,
            3 => 3360,
            _ => 15 * v_long_side(n - 1) - 15 * v_long_side(n - 2) + v_long_side(n - 3),
        }
    }

    const MAX: usize = 1_000_000_000;

    let mut p = 0;
    for i in 1.. {
        let temp_p = 3 * v_short_side(i) + 1;
        if temp_p + p > MAX { break } else { p += temp_p }

        let temp_p = 3 * v_long_side(i) + 2;
        if temp_p + p > MAX { break } else { p += temp_p }
    }

    assert_eq!(p, 518408346);
    format!("eu094 = {}", p)
} // 518408346

/// Amicable chains
pub fn eu095() -> String {
    // returns Some(length of amicable chain, min value in chain) or None
    fn is_amicable_chain(n: usize, divs: &mut Vec<usize>) -> Option<(usize, usize)> {
        let (mut next, mut cnt, mut min) = (n, 0, n);
        for _ in 0..1000 {
            match next {
                0 | 1 => return None,
                a if divs[a] >= divs.len() || a == divs[a] || a == divs[divs[a]] => return None,
                _ => {
                    cnt += 1;
                    next = divs[next];
                    if next < min {
                        min = next
                    }
                    if next == n {
                        return Some((cnt, min));
                    }
                }
            }
        }
        None
    }

    let max = 1_000_000;
    let mut divs = eu::divisor_sum_list(max);

    assert!(is_amicable_chain(28, &mut divs) == None);
    assert!(is_amicable_chain(284, &mut divs) == None);
    assert!(is_amicable_chain(562, &mut divs) == None);
    assert!(is_amicable_chain(12496, &mut divs) == Some((5, 12496)));
    assert!(is_amicable_chain(14316, &mut divs) == Some((28, 14316)));
    assert!(is_amicable_chain(138, &mut divs) == None);

    let amic_chains = (1..max)
                          .map(|x| (x, is_amicable_chain(x, &mut divs)))
                          .filter(|&(_, amic)| amic != None)
                          .map(|(x, tup)| (x, tup.unwrap().0, tup.unwrap().1))
                          .collect::<Vec<_>>();

    let max_cnt = amic_chains.iter()
                             .map(|&(_, cnt, _)| cnt)
                             .max()
                             .unwrap();

    let min = amic_chains.iter()
                         .filter(|&&(_, cnt, _)| cnt == max_cnt)
                         .map(|&(_, _, min)| min)
                         .min()
                         .unwrap();

    assert_eq!(min, 14316);
    format!("eu095 = {}", min)
} // 14316

/// Su Doku
pub fn eu096() -> String {

    #[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
    struct Cell {
        v: char,
        possible: [char; 9],
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut str = "".to_string();
            for v in self.possible.iter() {
                if *v != '0' { str = str + &format!("{}", v) } else { str = str + &format!(" ") }
            }
            let mut _v = ' ';
            if self.v != '0' { _v = self.v } else { _v = '_' }
            write!(f, "[{}, {}]", _v, str)
        }
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
    struct Row {
        c: [Cell; 9],
    }

    impl fmt::Display for Row {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut str = "".to_string();
            for v in self.c.iter() {
                str = str + &format!("{} ", v)
            }
            write!(f, "[ {}]", str)
        }
    }


    #[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
    struct Grid {
        r: [Row; 9],
    }

    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut str = "           1              2              3              4              5".to_string();
            str = str + &"              6              7             8              9\n".to_string();
            for (i, v) in self.r.iter().enumerate() {
                str = str + &format!("{} {}\n", i + 1, v)
            }
            write!(f, "{}", str)
        }
    }

    impl Grid {
        fn new() -> Grid {
            let cell = Cell { v: '0', possible: ['1', '2', '3', '4', '5', '6', '7', '8', '9'] };
            let row = Row { c: [cell; 9] };
            Grid { r: [row; 9] }
        }

        // take a text game vector and transform it to a new game grid
        fn new_game(xss: Vec<Vec<char>>) -> Grid {
            let mut g = Grid::new();
            for i in 0..9 {
                for j in 0..9 {
                    g.r[i].c[j].v = xss[i][j]
                }
            }
            g
        }

        // adjust cell.possible to reflect only posible values for cell.v
        fn adjust_possible_rows_cols(&mut self) {
            for i in 0..9 {
                for j in 0..9 {
                    // adjust for value in cell
                    if self.r[i].c[j].v != '0' {
                        self.r[i].c[j].possible = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
                        continue;
                    }
                    for k in 0..9 {
                        // adjust for values in row i, col 0..9
                        if self.r[i].c[k].v != '0' {
                            self.r[i].c[j].possible[self.r[i].c[k].v as usize - 49] = ' '
                        }
                        // adjust for values in row 0..9, col j
                        if self.r[k].c[j].v != '0' {
                            self.r[i].c[j].possible[self.r[k].c[j].v as usize - 49] = ' '
                        }
                    }
                }
            }
        }

        fn adjust_possible_boxes(&mut self) {
            // return vector,  9 rows of (r,c) pairs for location of boxes
            let xss: Vec<Vec<(usize, usize)>> = vec![
                    vec![(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2)],
                    vec![(0,3),(0,4),(0,5),(1,3),(1,4),(1,5),(2,3),(2,4),(2,5)],
                    vec![(0,6),(0,7),(0,8),(1,6),(1,7),(1,8),(2,6),(2,7),(2,8)],
                    vec![(3,0),(3,1),(3,2),(4,0),(4,1),(4,2),(5,0),(5,1),(5,2)],
                    vec![(3,3),(3,4),(3,5),(4,3),(4,4),(4,5),(5,3),(5,4),(5,5)],
                    vec![(3,6),(3,7),(3,8),(4,6),(4,7),(4,8),(5,6),(5,7),(5,8)],
                    vec![(6,0),(6,1),(6,2),(7,0),(7,1),(7,2),(8,0),(8,1),(8,2)],
                    vec![(6,3),(6,4),(6,5),(7,3),(7,4),(7,5),(8,3),(8,4),(8,5)],
                    vec![(6,6),(6,7),(6,8),(7,6),(7,7),(7,8),(8,6),(8,7),(8,8)],
                ];

            for xs in xss.clone() {
                for x in xs.clone() {
                    let (i, j) = x;
                    for z in xs.clone() {
                        let (m, n) = z;
                        if self.r[m].c[n].v != '0' {
                            self.r[i].c[j].possible[self.r[m].c[n].v as usize - 49] = ' '
                        }
                    }
                }
            }

        }

        fn cnt_poss(&self, row: usize, col: usize) -> u32 {
            self.r[row].c[col]
                .possible
                .into_iter()
                .fold(0, |acc, &x| if x != ' ' { acc + 1 } else { acc })
        }

        // find cell with minimum possible count
        fn find_min_cell(&self) -> (usize, usize) {
            let (mut min_i, mut min_j, mut v) = (0, 0, 9);
            for i in 0..9 {
                for j in 0..9 {
                    let temp = self.cnt_poss(i, j);
                    if temp != 0 && temp < v {
                        v = temp;
                        min_i = i;
                        min_j = j
                    }
                }
            }
            (min_i, min_j)
        }

        fn is_solved(&self) -> bool {
            if !self.is_valid_grid() {
                return false;
            }
            for i in 0..9 {
                for j in 0..9 {
                    if self.r[i].c[j].v == '0' {
                        return false;
                    }
                }
            }
            true
        }

        fn is_valid_grid(&self) -> bool {

            fn len_ok(cs: &Vec<char>) -> bool {
                let mut ts = cs.iter().filter(|&x| *x != '0').collect::<Vec<&char>>();
                let l = ts.len();
                ts.sort();
                ts.dedup();
                if l != ts.len() {
                    return false;
                }
                true
            }

            let empty_poss = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
            let mut cs_rows: Vec<char> = Vec::new();
            let mut cs_cols: Vec<char> = Vec::new();
            let mut cs_boxes: Vec<char> = Vec::new();
            for i in 0..9 {
                for j in 0..9 {
                    if self.r[i].c[j].possible == empty_poss && self.r[i].c[j].v == '0' {
                        return false;
                    }
                    cs_rows.push(self.r[i].c[j].v);
                    cs_cols.push(self.r[j].c[i].v);
                    cs_boxes.push(self.r[(i / 3) * 3 + j / 3].c[i * 3 % 9 + j % 3].v);
                }
                if !len_ok(&mut cs_rows) { return false } else { cs_rows.clear() }
                if !len_ok(&mut cs_cols) { return false } else { cs_cols.clear() }
                if !len_ok(&mut cs_boxes) { return false } else { cs_boxes.clear() }
            }
            true
        }

        fn update(&mut self, row: usize, col: usize, mut pos: usize) {
            for k in 0..9 {
                if self.r[row].c[col].possible[k] != ' ' {
                    if pos == 1 {
                        self.r[row].c[col].v = self.r[row].c[col].possible[k];
                        self.r[row].c[col].possible[k] = ' ';
                        return;
                    } else {
                        pos -= 1;
                    }
                }
            }
        }

        fn solve(&self) -> Grid {
            let grid = &mut self.clone();
            grid.adjust_possible_rows_cols();
            grid.adjust_possible_boxes();
            if grid.is_solved() {
                return *grid;
            }
            if !self.is_valid_grid() {
                return *self;
            }
            loop {
                let (i, j) = grid.find_min_cell();
                let min_cnt_poss = grid.cnt_poss(i, j);
                if min_cnt_poss == 1 {
                    grid.update(i, j, 1);
                    return grid.solve();
                }
                let mut grid_copy = grid.clone();
                grid_copy.update(i, j, 1);
                grid_copy = grid_copy.solve();
                if grid_copy.is_solved() {
                    return grid_copy;
                }
                grid.update(i, j, 1);

            }
        }
    }

    let mut xss: Vec<Vec<Vec<char>>> = Vec::new();
    let mut temp: Vec<Vec<char>> = Vec::new();
    let mut cnt = 0;
    for v in include_str!("../data/p096_sudoku.txt").lines() {
        let cs = v.chars().collect::<Vec<_>>();
        if cs[0] == 'G' {
            continue;
        }
        temp.push(cs);
        cnt += 1;
        if cnt == 9 {
            xss.push(temp.clone());
            temp.clear();
            cnt = 0;
        }
    }

    let mut cnt = 0;
    for v in xss.iter() {
        // for (idx, v) in xss.iter().enumerate() {
        // println!("solving Sudoku puzzle {}...", idx + 1);
        let grid = Grid::new_game(v.clone());
        let aaa = grid.solve();
        cnt += 100 * aaa.r[0].c[0].v.to_digit(10).unwrap() + 10 * aaa.r[0].c[1].v.to_digit(10).unwrap() +
               aaa.r[0].c[2].v.to_digit(10).unwrap();
        // println!("{}", aaa);
    }

    assert_eq!(cnt, 24702);
    format!("eu096 = {}", cnt)
} // 24702


/// Large non-Mersenne prime
pub fn eu097() -> String {
    let res: usize = (0..7830457).fold(28433, |acc, _| (2 * acc) % 10_000_000_000) + 1;
    assert_eq!(res, 8739992577);
    format!("eu097 = {}", res)
} // 8739992577

/// Anagramic squares
pub fn eu098() -> String {
    // get the words from the file
    fn get_words() -> Vec<String> {
        let words = include_str!("../data/p098_words.txt")
                        .chars()
                        .filter(|&x| x != '\"')
                        .collect::<String>()
                        .split(',')
                        .map(|x| x.to_string())
                        .collect_vec();
        words
    }

    // sort each word within words, save words original index
    fn get_sorted_words(words: Vec<String>) -> Vec<(Vec<char>, usize)> {
        let sorted_words = words.into_iter()
                                .enumerate()
                                .map(|(i, x)| (x.chars().sorted(), i))
                                .sorted();
        sorted_words
    }

    // find anagram pairs
    fn get_anagrams(words: Vec<String>) -> Vec<(usize, usize)> {
        let sws = get_sorted_words(words);
        let anagrams = (0..sws.len() - 1)
                           .filter(|&i| sws[i].0 == sws[i + 1].0)
                           .map(|i| (sws[i].1, sws[i + 1].1))
                           .collect();
        anagrams
    }

    // vector of i*i exactly n digits long
    fn get_squares(n: usize) -> Vec<usize> {
        let mut squares: Vec<usize> = Vec::new();
        let from = (10.0 as f64).powf((n - 1) as f64).sqrt() as usize;
        let to = (10.0 as f64).powf((n) as f64).sqrt() as usize;
        for i in from..to {
            squares.push(i * i);
        }
        squares
    }

    // get_pattern([a,b,c,b,d]) == [None,Some(3),None,Some(1),None]
    fn get_pattern(cs: &Vec<char>) -> Vec<Option<usize>> {
        let mut res = vec![None; cs.len()];
        for i in 0..cs.len() - 1 {
            for j in i + 1..cs.len() {
                if cs[i] == cs[j] {
                    res[i] = Some(j);
                    res[j] = Some(i)
                }
            }
        }
        res
    }

    // find cnadidate squared number
    fn find_candidate(w1: &String, w2: &String, num: usize) -> Option<usize> {
        let mut hash = HashMap::new();
        let ws1 = w1.chars().collect_vec();
        let nums = &num.to_string().chars().collect_vec();
        if get_pattern(&ws1) != get_pattern(&nums) {
            return None;
        }
        for i in 0..ws1.len() {
            hash.insert(ws1[i], nums[i]);
        }
        let ws2 = w2.chars().collect_vec();
        let mut res = vec!['0'; ws2.len()];
        for i in 0..ws2.len() {
            res[i] = *hash.get(&ws2[i]).unwrap()
        }
        let candidate = res.into_iter().collect::<String>().parse::<usize>().unwrap();
        Some(candidate)
    }

    let words = get_words();
    let all_anagrams = get_anagrams(words.clone());
    let mut max_anagram = 0;
    for (v, _) in all_anagrams.clone() {
        if words[v].len() > max_anagram {
            max_anagram = words[v].len()
        }
    }

    let mut res: usize = 0;
    for i in 5..max_anagram + 1 {
        let anagrams = all_anagrams.iter()
                                   .filter(|&&(x, _)| words[x].len() == i)
                                   .collect_vec();
        let squares = get_squares(i);

        for j in 0..anagrams.len() {
            for n in squares.clone() {
                let candidate = find_candidate(&words[anagrams[j].0], &words[anagrams[j].1], n);
                if candidate == None {
                    continue;
                }
                let candidate = candidate.unwrap();
                if candidate != n && squares.contains(&candidate) {
                    let max = cmp::max(candidate, n);
                    if max > res {
                        res = max
                    }
                }
            }
        }
    }

    assert_eq!(res, 18769);
    format!("eu098 = {}", res)
} // 18769

/// Largest exponential
pub fn eu099() -> String {
    // logb(x^y) = y ∙ logb(x)
    let buffer = include_str!("../data/p099_base_exp.txt");
    let xs = buffer.lines()
                   .map(|x| {
                       x.split(',')
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect_vec()
                   })
                   .enumerate()
                   .map(|(i, pair)| ((pair[1] * pair[0].ln()).to_string(), i))
                   .sorted();

    let res = xs.last().unwrap().1 + 1;
    assert_eq!(res, 709);
    format!("eu099 = {}", res)
} // 709

/// Arranged probability
pub fn eu100() -> String {
    // https://www.alpertron.com.ar/QUAD.HTM
    // (b/n)*((b-1)/(n-1)) = 2b^2 - 2b - n^2 + n = 0
    // [a,b,c,d,e,f]=[2,0,-1,-2,1,0]
    // [P,Q,K,R,S,L]=[3,2,-2,4,3,-3]
    let mut max = 0.0;
    let mut b = 15.0;
    let mut n = 21.0;
    while max < 1_000_000_000_000.0 {
        let b_new = 3.0 * b + 2.0 * n - 2.0;
        n = 4.0 * b + 3.0 * n - 3.0;
        b = b_new;
        max += n;
    }

    assert_eq!(b, 756872327473.0);
    format!("eu100 = {}", b)
} // 756872327473

/// Returns Vec of the Euler solution functions in this crate.
pub fn get_functions() -> Vec<fn() -> String> {
    vec![eu091, eu092, eu093, eu094, eu095, eu096, eu097, eu098, eu099, eu100]
}
