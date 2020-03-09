impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut i = (area as f64).sqrt() as i32;
        while i > 1 {
            if area % i == 0 {
                break;
            }
            i -= 1;
        }
        vec![area / i, i]
    }
}
