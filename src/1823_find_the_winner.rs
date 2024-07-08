use std::collections::VecDeque;
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut v: VecDeque<_> = (1..=n).collect();
        while v.len() > 1 {
            let offset = (k - 1) % (v.len() as i32);
            for _ in 0..offset {
                let n = v.pop_front().unwrap();
                v.push_back(n);
            }
            let n = v.pop_front();
        }
        v.pop_front().unwrap()
    }
}
