pub struct Solution;
impl Solution {
    pub fn largest_sum_after_k_negations(mut a: Vec<i32>, mut k: i32) -> i32 {
        a.sort();
        for i in 0..a.len() {
            if k == 0 {
                break;
            }
            if a[i] >=0 {
                if k % 2 == 1 {
                    let j = if i > 0 && a[i] > a[i-1] {
                        i - 1
                    } else { i };
                    a[j] = -a[j];
                }
                break;
            } else {
                a[i] = -a[i];
                k -= 1;
            }
        }
        a.iter().sum()
    }
}
