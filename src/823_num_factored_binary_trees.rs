const M: i64 = 1_000_000_007;
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        arr.sort();
        for num in arr {
            let mut cnt = 1;
            for (&q, &left_cnt) in &map {
                if num % q != 0 { continue; }
                if let Some(right_cnt) = map.get(&(num / q)) {
                    cnt += (left_cnt * right_cnt) % M;
                    cnt %= M;
                }
            }
            map.insert(num, cnt);
        }
        let mut ret = 0;
        for cnt in map.into_values() {
            ret = (ret + cnt) % M;
        }
        ret as i32
    }
}
