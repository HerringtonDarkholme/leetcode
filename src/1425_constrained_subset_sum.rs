use std::collections::VecDeque;
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        // accept subseq with higher sum or later index
        let mut deque: VecDeque<(usize, i32)> = VecDeque::new();
        let mut max = i32::MIN;
        for (i, n) in nums.into_iter().enumerate() {
            if !deque.is_empty() { // pop element out out of range
                if i - deque[0].0 > k as usize {
                    deque.pop_front();
                }
            }
            let curr = if deque.is_empty() { 0 } else { deque[0].1 } + n;
            max = max.max(curr);
            if curr < 0 { continue; } // no need to add sum < 0 in stack
            while !deque.is_empty() && deque[deque.len() - 1].1 < curr { 
                // pop subseq with less sum
                deque.pop_back();
            }
            deque.push_back((i, curr));
        }
        max
    }
}

// 37 
