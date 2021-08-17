use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut c = HashMap::new();
        for n in arr {
            if c.contains_key(&n) {
                let m = c.get_mut(&n).unwrap();
                *m -= 1;
                if *m == 0 {
                    c.remove(&n);
                }
                continue;
            }
            if n < 0 {
                if n % 2 != 0 {
                    return false
                }
                *c.entry(n / 2).or_insert(0) += 1;
            } else {
                *c.entry(n * 2).or_insert(0) += 1;
            }
        }
        c.is_empty()
    }
}
