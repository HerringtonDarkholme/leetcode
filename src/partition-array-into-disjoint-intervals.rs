pub struct Solution;

impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut mins = vec![a[a.len() - 1]; a.len()];
        let mut maxs = vec![a[0]; a.len()];
        for i in 1..a.len() {
            maxs[i] = maxs[i - 1].max(a[i]);
            mins[a.len() - i - 1] = mins[a.len() - i].min(a[a.len() - i - 1]);
        }
        for i in 1..a.len() {
            if maxs[i - 1] <= mins[i] {
                return i as i32;
            }
        }
        0 as i32
    }
}
