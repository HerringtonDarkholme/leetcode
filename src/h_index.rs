// leetcode 274
pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut buckets = vec![0; len + 1];
        // bucket sort, O(n) sort where elements are known in buckets
        for c in citations {
            if c > len as i32 {
                buckets[len] += 1;
            } else {
                buckets[c as usize] += 1;
            }
        }
        let mut n = 0;
        for (c, &num) in buckets.iter().enumerate().rev() {
            n += num;
            if n >= c {
                return c as i32
            }
        }
        0
    }
}

/*
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
*/
