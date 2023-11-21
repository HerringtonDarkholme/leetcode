use std::collections::HashMap;

const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = 0i64;
        let mut map = HashMap::new();
        for n in nums {
            let n = compute(n);
            if let Some(pair) = map.get_mut(&n) {
                cnt += *pair;
                cnt %= M;
                *pair += 1;
            } else {
                map.insert(n, 1);
            }
        }
        cnt as i32
    }
}
fn compute(n: i32) -> i32 {
    let mut rev = 0;
    let mut m = n;
    while m != 0 {
        rev = rev * 10 + m % 10;
        m /= 10;
    }
    n - rev
}
