pub struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut stack = vec![];
        for n in nums.into_iter().rev() {
            let n = n as isize;
            let index = match stack.binary_search(&(n-1)) {
                Ok(n) => n + 1,
                Err(n) => n,
            };
            r += index;
            let index = match stack.binary_search(&(n * 2)) {
                Ok(n) => n,
                Err(n) => n,
            };
            stack.insert(index, n * 2);
        }
        r as i32
    }
}
