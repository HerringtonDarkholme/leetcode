pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut r = vec![];
        for n in nums.into_iter().rev() {
            let index = match r.binary_search(&(n-1)) {
                Ok(i) => i+1,
                Err(i) => i,
            };
            ret.push(index as i32);
            r.insert(index, n);
        }
        ret.into_iter().rev().collect()
    }
}
