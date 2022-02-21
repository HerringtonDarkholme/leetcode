use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut map = HashMap::new();
        let k = k as i64;
        let mut ret = 0;
        for n in nums {
            let g = gcd(n as i64, k);
            for (&other, &cnt) in map.iter() {
                if (g * other) % k == 0 {
                    ret += cnt;
                }
            }
            *map.entry(g).or_insert(0) += 1;
        }
        ret
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        gcd(b, a)
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
