pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        aux(k, n, 1)
    }
}
fn aux(k: i32, n: i32, start: i32) -> Vec<Vec<i32>> {
    if k == 0 || start > 9 {
        return vec![]
    }
    if k == 1 && n <= 9 && n >= start {
        return vec![vec![n]]
    }
    let mut ret = vec![];
    for i in start..10 {
        for mut v in aux(k - 1, n - i, i + 1) {
            v.insert(0, i);
            ret.push(v);
        }
    }
    ret
}
