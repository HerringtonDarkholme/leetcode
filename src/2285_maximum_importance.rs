impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut counter = vec![0usize; n as usize];
        for road in roads {
            counter[road[0] as usize] += 1;
            counter[road[1] as usize] += 1;
        }
        counter.sort_unstable();
        counter.into_iter().enumerate().rev()
            .map(|(n, c)| c * (n + 1))
            .sum::<usize>() as i64
    }
}
