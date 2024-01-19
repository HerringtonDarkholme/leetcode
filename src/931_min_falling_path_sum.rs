use std::collections::{HashSet, BinaryHeap};
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = matrix[0].clone();
        for v in matrix.into_iter().skip(1) {
            let mut next = vec![i32::MAX; v.len()];
            for i in 0..v.len() {
                for j in [-1, 0, 1] {
                    let c = i as i32 + j;
                    if c < 0 || c >= v.len() as i32 {
                        continue;
                    }
                    next[i] = next[i].min(dp[c as usize] + v[i]);
                }
            }
            dp = next;
        }
        dp.into_iter().min().unwrap()
    }
}
