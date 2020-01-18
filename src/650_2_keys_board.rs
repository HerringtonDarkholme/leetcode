impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        min_step(n)
    }
}
fn min_step(n: i32) -> i32 {
    if n == 1 {
        0
    } else {
        let mut min = i32::max_value();
        for i in 1..n {
            if n % i == 0 {
                min = min.min(n / i) + min_step(i);   
            }
        }
        min
    }
}
