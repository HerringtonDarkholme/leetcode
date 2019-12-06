// 922
pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        while i < a.len() {
            if a[i] % 2 != 0 {
                a.swap(i, j);
                j += 2;
            } else {
                i += 2;
            }
        }
        a
    }
}
