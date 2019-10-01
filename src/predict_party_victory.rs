pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senates: Vec<_> = senate.chars().collect();
        let mut r = 0;
        let mut d = 0;
        let mut queue = VecDeque::new();
        for &c in senates.iter() {
            if c == 'R' {
                r += 1;
            } else {
                d += 1;
            }
            queue.push_back(c);
        }
        let mut prev_r = 0;
        let mut prev_d = 0;
        while r != 0 && d != 0 {
            let s = queue.pop_front().unwrap();
            if s == 'R' {
                if prev_d > 0 {
                    prev_d -= 1;
                    r -= 1;
                } else {
                    prev_r += 1;
                    queue.push_back(s);
                }
            } else {
                if prev_r > 0 {
                    prev_r -= 1;
                    d -= 1;
                } else {
                    prev_d += 1;
                    queue.push_back(s);
                }
            }
        }
        if r > 0 {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}
