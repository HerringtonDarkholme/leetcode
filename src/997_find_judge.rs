use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        for t in trust {
            *map.entry(t[1]).or_insert(0) += 1;
            set.insert(t[0]);
        }
        for (k, v) in map {
            if v == n - 1 && !set.contains(&k) {
                return k
            }
        }
        -1
    }
}
