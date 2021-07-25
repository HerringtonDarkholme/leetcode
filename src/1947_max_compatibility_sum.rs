struct  Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let table = compatibility_table(students, mentors);
        let mut visited = HashSet::new();
        max_compatibility_sum(&table, 0, &mut visited)
    }
}
fn max_compatibility_sum(table: &Vec<Vec<i32>>, i: usize, visited: &mut HashSet<usize>) -> i32 {
    let mut max = 0;
    if i >= table.len() {
        return max
    }
    for j in 0..table.len() {
        if visited.contains(&j) {
            continue;
        }
        visited.insert(j);
        max = max.max(table[i][j] + max_compatibility_sum(table, i+1, visited));
        visited.remove(&j);
    }
    max
}

fn compatibility_table(ss: Vec<Vec<i32>>, ms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let l = ss.len();
    let mut ret = vec![];
    for i in 0..l {
        let mut t = vec![];
        for j in 0..l {
            t.push(compute_compat(&ss[i], &ms[j]));
        }
        ret.push(t);
    }
    ret
}

fn compute_compat(s: &Vec<i32>, m: &Vec<i32>) -> i32 {
    s.iter().zip(m.iter()).map(|(a, b)| {
        1 - (a ^ b)
    }).sum()
}
