pub struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        }
        let mut ret = vec![];
        for i in 1..=9 {
            Solution::recur(i, n - 1, k, &mut ret);
        }
        ret
    }
    fn recur(i: i32, n: i32, k: i32, ret: &mut Vec<i32>) {
        if n == 0 {
            ret.push(i);
            return;
        }
        let last = i % 10;
        if last - k >= 0 {
            Solution::recur(i * 10 + last - k, n - 1, k, ret);
        }
        if last + k <= 9 && k != 0 {
            Solution::recur(i * 10 + last + k, n - 1, k, ret);
        }
    }
}
