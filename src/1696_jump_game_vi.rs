use std::collections::BTreeMap;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        assert!(nums.len() >= 1);
        let k = k as usize;
        let mut counter = BTreeMap::new();
        counter.insert(nums[0], 1);
        let mut results = vec![nums[0]];
        for i in 1..nums.len() {
            let (&cost, _) = counter.iter().rev().next().unwrap();
            let r = cost + nums[i];
            *counter.entry(r).or_insert(0) += 1;
            results.push(r);
            if i >= k {
                let cost = results[i - k];
                let k = counter.get_mut(&cost).unwrap();
                *k -= 1;
                if *k == 0 {
                    counter.remove(&cost);
                }
            }
        }
        *results.last().unwrap()
    }
}
