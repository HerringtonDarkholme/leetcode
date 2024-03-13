impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let sum = (n + 1) * n / 2;
        let m = (sum as f64).sqrt() as i32;
        if m * m == sum {
            m
        }  else {
            -1
        }
    }
}
