fn cmp(a: &str, b: &str) -> Ordering {
    let len = a.len().min(b.len());
    let sa = &a[..len];
    let sb = &b[..len];
    match sb.cmp(sa) {
        Ordering::Equal => {
            if a.len() == b.len() {
                Ordering::Equal
            } else if a.len() > b.len() {
                cmp(&a[b.len()..], sb)
            } else {
                cmp(sa, &b[a.len()..])
            }
        },
        o => o,
    }
}
use std::cmp::{Reverse, Ordering};
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.iter().all(|n| *n == 0) {
            return "0".into()
        }
        let mut nums: Vec<_> = nums.into_iter().map(|n| n.to_string()).collect();
        nums.sort_by(|a, b| cmp(a, b));
        nums.join("").to_string()
    }
}
