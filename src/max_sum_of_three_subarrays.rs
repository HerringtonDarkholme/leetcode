pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        // let mut stack = vec![];
        let sums: Vec<i32> = nums.windows(k).map(|k| k.iter().sum()).collect();
        let mut hashes = vec![HashMap::new(); 3];
        Solution::find_recur(0, k, 3, &sums, &mut hashes);
        hashes[2].get(&0).unwrap().iter().map(|&i| i as i32).collect()
    }
    pub fn find_recur(start: usize, k: usize, remaining: usize, sums: &Vec<i32>, hashes: &mut Vec<HashMap<usize, Vec<usize>>>) -> Vec<usize> {
        if remaining == 0 {
            return vec![]
        }
        if let Some(r) = hashes[remaining - 1].get(&start) {
            return r.clone()
        }
        let mut max = 0;
        let mut max_vec = vec![];
        let mut i = start;
        let limit = sums.len() - k * (remaining - 1);
        while i <  limit {
            let mut ret = Solution::find_recur(i + k, k, remaining - 1, sums, hashes);
            let current = sums[i] + ret.iter().map(|&i| sums[i]).sum::<i32>();
            let next_i = if !ret.is_empty() {
                let mut j = i + 1;
                let mut mj = j;
                let mut ms = sums[i];
                while j < limit && j < ret[0] - k {
                    if sums[j] > ms {
                        ms = sums[j];
                        mj = j;
                    }
                    j += 1;
                }
                mj
            } else {
                i + 1
            };
            if  current > max {
                max = current;
                ret.insert(0, i);
                max_vec = ret;
            }
            i = next_i;
        }
        hashes[remaining - 1].insert(start, max_vec.clone());
        max_vec
    }
}

#[test]
fn test() {
    for i in vec![
        (vec![1,2,1,2,6,7,5,1], 2, vec![0,3,5]),
        (vec![3,43,4,2,2,6,7,3,23,3,5,6,8,9,34,23,4,4,6,8,4,233,54,5,34,34,4,56,45654], 3, vec![20, 23, 26]),
        (vec![3,3,4,6,2,3,5,8,2,5,3,6,8,3,7,8,9,9,3,2,1,5,5,6,8,2,1,2,3,5], 5, vec![9, 14, 21]),
        (vec![1,2,3], 1, vec![0, 1, 2]),
    ] {
        assert_eq!(Solution::max_sum_of_three_subarrays(i.0, i.1), i.2);
    }
}
