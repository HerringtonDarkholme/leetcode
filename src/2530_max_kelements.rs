use std::collections::BinaryHeap;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut heap: BinaryHeap<_> = nums.into_iter().collect();
        let mut ret = 0;
        while k > 0 {
            let next = heap.pop().unwrap();
            ret += next as i64;
            heap.push((next + 2) / 3);
            k -= 1;
        }
        ret
    }
}
