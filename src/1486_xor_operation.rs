impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |a, b| a ^ (start + b * 2))
    }
}
