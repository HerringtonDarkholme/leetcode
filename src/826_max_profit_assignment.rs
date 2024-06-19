use std::collections::BTreeMap;
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut tasks: Vec<_> = difficulty.into_iter().zip(profit.into_iter()).collect();
        tasks.sort();
        let mut profits = BTreeMap::new();
        let mut max = 0;
        for (d, p) in tasks {
            max = p.max(max);
            profits.insert(d, max);
        }
        let mut ret = 0;
        for w in worker {
            ret += profits.range(..=w).map(|n| *n.1).rev().next().unwrap_or(0);
        }
        ret
    }
}
