use std::collections::HashSet;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool { 
        let c = c as i64;
        let mut ret = HashSet::new();
        for i in 0..=c {
            if i * i > c {
                break;
            }
            let n = i * i;
            ret.insert(n);
            if ret.contains(&(c - n)) {
                return true;
            }
        }
        false
    }
}
