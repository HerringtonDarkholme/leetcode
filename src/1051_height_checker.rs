impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();
        expected.into_iter().zip(heights.into_iter()).filter(|(a, b)| a != b).count() as i32
    }
}
