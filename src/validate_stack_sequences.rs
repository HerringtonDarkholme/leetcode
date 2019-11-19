// leetcode 946
pub struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pop_i = 0;
        let len = pushed.len();
        let mut stack = vec![];
        for p in pushed {
            stack.push(p);
            while !stack.is_empty() && *stack.last().unwrap() == popped[pop_i] {
                pop_i += 1;
                stack.pop();
            }
        }
        pop_i == len
    }
}
