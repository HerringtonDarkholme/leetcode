use std::iter::repeat;
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let len = s.len();
        let ones = s.chars().filter(|&c| c == '1').count();
        repeat('1').take(ones - 1)
            .chain(repeat('0').take(len - ones))
            .chain(std::iter::once('1'))
            .collect()
    }
}
