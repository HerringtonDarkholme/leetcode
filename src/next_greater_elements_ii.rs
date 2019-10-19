pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![-1; nums.len()];
        let l = nums.len();
        for i in 0..l*2 {
            let i = i % l;
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                let j = stack.pop().unwrap();
                ret[j] = nums[i];
            }
            stack.push(i);
        }
        ret
    }
}
