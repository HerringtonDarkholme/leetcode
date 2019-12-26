pub struct Solution;

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        reach_number(target.abs() as i32)
    }
}

fn reach_number(target: i32) -> i32 {
    let mut n = ((target * 2) as f64).sqrt() as i32;
    let mut s = n * (n + 1) / 2;
    if s < target {
        n += 1;
        s = n * (n + 1) / 2;
    }
    if target == s {
        n
    } else if (s - target) % 2 == 0 { 
        n 
    } else if n % 2 == 0 { 
        n + 1
    } else {
        n + 2
    }
}
