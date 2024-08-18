use std::collections::BTreeSet;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut set = BTreeSet::new();
        set.insert(1usize);
        for i in 0..n {
            let num = set.pop_first().unwrap();
            if i == n - 1 {
                return num as i32
            }
            set.insert(num * 2);
            set.insert(num * 3);
            set.insert(num * 5);
        }
        -1
    }
}
