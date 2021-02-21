
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .max()
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    }
}
