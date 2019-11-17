// leetcode 274
pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let mut h = 0;
        let len = citations.len();
        for i in 0..len {
            if citations[len - 1 - i] >= (i + 1) as i32 {
                h = i as i32 + 1;
            }
        }
        h
    }
}
