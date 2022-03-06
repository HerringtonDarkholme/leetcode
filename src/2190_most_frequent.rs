use std::collections::HashMap;
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = HashMap::new();
        for i in 1..nums.len() {
            if nums[i - 1] == key {
                *map.entry(nums[i]).or_insert(0) += 1;
            }
        }
        map.into_iter().map(|v| (v.1, v.0)).max().unwrap().1
    }
}

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&n| rewrite(n, &mapping));
        nums
    }
}

fn rewrite(mut n: i32, mapping: &[i32]) -> i32 {
    let mut new = 0;
    if n == 0 {
        return mapping[0];
    }
    let mut i = 1;
    while n != 0 {
        new += mapping[(n % 10) as usize] * i;
        i *= 10;
        n /= 10;
    }
    new
}

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut parents = get_parents(n as usize, edges);
        let mut ancestors = vec![vec![]; n as usize];
        for i in 0..n {
            get_ancestors(i as usize, &parents, &mut ancestors);
        }
        ancestors
    }
}

fn get_parents(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut parents = vec![vec![]; n];
    for edge in edges {
        parents[edge[1] as usize].push(edge[0]);
    }
    parents
}

use std::collections::BTreeSet;
fn get_ancestors(i: usize, p: &Vec<Vec<i32>>, ancestors: &mut Vec<Vec<i32>>) {
    if !ancestors[i].is_empty() {
        return;
    }
    let mut ans = BTreeSet::new();
    for &n in &p[i] {
        ans.insert(n);
        get_ancestors(n as usize, p, ancestors);
        for &aa in &ancestors[n as usize] {
            ans.insert(aa);
        }
    }
    ancestors[i] = ans.into_iter().collect();
}
