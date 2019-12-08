// leetcode 769
pub struct Solution;

impl Solution {
    // the first chunk means the largest element in sub slice: arr[:k+1] is k
    // so all element in the subslice is 0..k
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max_idx = 0;
        let mut chunk = 0;
        for i in 0..arr.len() {
            max_idx = max_idx.max(arr[i]);
            if max_idx as usize == i {
                chunk += 1;
            }
        }
        chunk
    }
}
