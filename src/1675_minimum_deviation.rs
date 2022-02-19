use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut min = i32::MAX;
        for mut i in nums {
            if i % 2 == 1 {
                i *= 2;
            }
            min = min.min(i);
            pq.push(i);
        }
        let mut diff = i32::MAX;
        while let Some(&max) = pq.peek() {
            if max % 2 != 0 {
                break;
            }
            pq.pop();
            diff = diff.min(max - min);
            min = min.min(max / 2);
            pq.push(max / 2);
        }
        diff.min(pq.pop().unwrap() - min)
    }
}
