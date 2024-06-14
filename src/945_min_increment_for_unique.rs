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
/*
impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut prev = -1;
        let mut ret = 0;
        for n in nums {
            ret += (prev + 1 - n).max(0);
            prev = n.max(prev + 1);
        }
        ret
    }
}
*/
