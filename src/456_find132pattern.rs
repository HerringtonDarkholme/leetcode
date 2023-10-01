
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut temp = i32::MIN;
        let mut stack = vec![];
        for &n in nums.iter().rev() {
            if n < temp {
                return true;
            }
            while !stack.is_empty() && stack[stack.len() - 1] < n {
                temp = stack.pop().unwrap();
            }
            stack.push(n);
        }
        false
    }
}
