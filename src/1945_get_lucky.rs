struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let start: Vec<_> = s.as_bytes().iter().map(|&c| {
            let a = (c - 'a' as u8 + 1) as i32;
            (a / 10) + (a % 10)
        }).collect();
        let mut sum = start.iter().sum();
        for _ in 1..k {
            let mut next = 0;
            while sum > 0 {
                next += sum % 10;
                sum /= 10;
            }
            sum = next;
        }
        sum
    }
}
