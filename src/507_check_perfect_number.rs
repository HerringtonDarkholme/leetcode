impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num == (1..num).filter(|f| num % f == 0).sum::<i32>()
    }
}
