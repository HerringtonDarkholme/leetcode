#[macro_use]
mod util;
#[path = "./recursive_remove_adjacent_duplicates.rs"]
pub mod remove_adj;

fn main() {}

#[test]
fn test() {}

struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                let need_one = i == j || i + j == n - 1;
                let is_zero = grid[i][j] == 0;
                if need_one && !is_zero || !need_one && is_zero {
                    continue;
                }
                return false;
            }
        }
        true
    }
}

const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n = n as i64;
        let mut total = 1;
        let mut unplaced = 1;
        for _ in 0..n {
            let next_placed = unplaced;
            let next_unplaced = total;
            total = (next_placed + next_unplaced) % M;
            unplaced = next_unplaced;
        }
        ((total * total) % M) as i32
    }
}

// total(n) = placed(n) + unplaced(n)
// placed(n) = unplaced(n-1)
// unplaced(n) = total(n-1)
// total(n) = unplaced(n-1) + total(n - 1)

impl Solution {
    pub fn maximums_spliced_array(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let sum1: i32 = nums1.iter().sum();
        let sum2: i32 = nums2.iter().sum();
        let mut max1 = 0;
        let mut acc1 = 0;
        let mut max2 = 0;
        let mut acc2 = 0;
        for i in 0..nums1.len() {
            let diff = nums2[i] - nums1[i];
            acc1 += diff;
            acc2 -= diff;
            max1 = max1.max(acc1);
            max2 = max2.max(acc2);
            acc1 = acc1.max(0);
            acc2 = acc2.max(0);
        }
        (max1 + sum1).max(max2 + sum2)
    }
}

/*
[28,34,38,14,30,31,23, 7,28, 3] 236
[42,35, 7, 6,24,30,14,21,20,34] 233
 */

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        for i in 0..edges.len() {
            for j in 0..edges.len() {
                if i == j {
                    continue;
                }
                min = min.min(check(&nums, &edges, i, j));
            }
        }
        min
    }
}

use std::collections::HashMap;
fn check(nums: &[i32], edges: &[Vec<i32>], exclude1: usize, exclude2: usize) -> i32 {
    let mut dsu = DSU::new(nums.len());
    for i in 0..edges.len() {
        if i == exclude1 || i == exclude2 {
            continue;
        }
        dsu.union(edges[i][0] as usize, edges[i][1] as usize);
    }
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let p = dsu.find(i);
        *map.entry(p).or_insert(0) ^= nums[i];
    }
    let max = *map.values().max().unwrap();
    let min = *map.values().min().unwrap();
    max - min
}

pub struct DSU {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>
}

impl DSU {
    pub fn new(size: usize) -> Self {
        DSU {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (xr, yr) = (self.find(x), self.find(y));
        if xr == yr {
            return false
        }
        if self.rank[xr] < self.rank[yr] {
            self.parent[xr] = yr;
        } else if self.rank[xr] > self.rank[yr] {
            self.parent[yr] = xr
        } else {
            self.parent[yr] = xr;
            self.rank[xr] += 1;
        }
        true
    }
}
