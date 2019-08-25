pub struct Solution;

use std::collections::{HashSet};
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let start = "0".repeat(n);
        let mut current = start.clone();
        let mut ret = current.clone();
        let mut set = HashSet::new();
        set.insert(start.clone());
        'outer: loop {
            for i in (0..k).rev() {
                let mut next = current.clone();
                let c = ('0' as u8 + i as u8) as char;
                next.push(c);
                let next = next.split_at(1).1;
                if set.contains(next) {
                    continue;
                }
                set.insert(next.to_owned());
                ret.push(c);
                current = next.to_owned();
                continue 'outer;
            }
            break;
        }
        ret
    }
}
