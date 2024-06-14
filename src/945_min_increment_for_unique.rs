pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
        if a.is_empty() {
            return 0
        }
        a.sort();
        let mut ret = 0;
        for i in 1..a.len() {
            if a[i] <= a[i - 1] {
                ret += a[i - 1] - a[i] + 1;
                a[i] = a[i - 1] + 1;
            }

        }
        ret
    }
}
