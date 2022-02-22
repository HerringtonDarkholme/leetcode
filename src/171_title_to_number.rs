impl Solution {
    pub fn title_to_number(title: String) -> i32 {
        title.bytes().fold(0, |a, b| a * 26 + (b - b'A' + 1) as i32)   
    }
}
