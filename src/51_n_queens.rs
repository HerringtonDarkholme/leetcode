pub struct Solution;

use std::collections::HashSet;
struct Line {
    q: i32,
    i: i32,
    n: i32,
}
impl Line {
    #[inline]
    fn new(q: i32, n: i32) -> Self {
        Line {
            q, n,
            i: 0,
        }
    }
}

impl Iterator for Line {
    type Item =char;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i >= self.n {
            None
        } else {
            self.i += 1;
            if i == self.q {
                Some('Q')
            } else {
                Some('.')
            }
        }
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut row = vec![];
        let mut col = HashSet::new();
        let mut sum = HashSet::new();
        let mut diff = HashSet::new();
        let mut ret = vec![];
        Solution::aux(n, &mut row, &mut col, &mut sum, &mut diff, &mut ret);
        ret.into_iter().map(|v| {
            v.into_iter().map(|i| {
                Line::new(i, n).collect()
            }).collect()
        }).collect()
    }

    fn aux(n: i32, row: &mut Vec<i32>, col: &mut HashSet<i32>, sum: &mut HashSet<i32>, diff: &mut HashSet<i32>, ret: &mut Vec<Vec<i32>>) {
        let r = row.len() as i32;
        if n == r {
            ret.push(row.clone());
            return
        }
        for c in 0..n {
            let s = c + r;
            let d = c - r;
            if col.contains(&c) || sum.contains(&s) || diff.contains(&d) {
                continue;
            }
            col.insert(c);
            sum.insert(s);
            diff.insert(d);
            row.push(c);
            Solution::aux(n, row, col, sum, diff, ret);
            col.remove(&c);
            sum.remove(&s);
            diff.remove(&d);
            row.pop();
        }
    }
}

#[test]
fn test() {
    dbg!(Solution::solve_n_queens(2));
    dbg!(Solution::solve_n_queens(4));
}

/*
 *
 *
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut pos = vec![];
        let mut ret = vec![];
        solve(n, &mut pos, 0, 0, 0, &mut ret);
        ret.into_iter().map(to_ans).collect()
    }
}

fn solve(n: usize, pos: &mut Vec<usize>, col: i32, diag1: i32, diag2: i32, ret: &mut Vec<Vec<usize>>) {
    if pos.len() == n {
        ret.push(pos.clone());
        return;
    }
    let r = pos.len();
    for c in 0..n {
        if (1 << c) & col != 0 {
            continue;
        }
        let d1 = r - c + n;
        if (1 << d1) & diag1 != 0 {
            continue;
        }
        if (1 << (r + c)) & diag2 != 0 {
            continue;
        }
        pos.push(c);
        solve(n, pos, col | (1 << c), diag1 | (1 << d1) , diag2 | (1 << (r + c)), ret);
        pos.pop();
    }
}

fn to_ans(r: Vec<usize>) -> Vec<String> {
    let mut ret = vec![vec!['.'; r.len()]; r.len()];
    for (r, c) in r.into_iter().enumerate() {
        ret[r][c] = 'Q';
    }
    ret.into_iter().map(|r| r.into_iter().collect()).collect()
}
 */
