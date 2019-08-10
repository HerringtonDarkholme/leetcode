pub struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        if a[0] == a[1] {
            return a[0]
        }
        for i in 1..(a.len() / 2) {
            let n2 = a[2*i];
            let n3 = a[2*i + 1];
            if n2 == n3 {
                return n2
            }
            if n2 == a[2*i-1] || n2 == a[2*i-2] {
                return n2
            }
            if n3 == a[2*i-1] || n3 == a[2*i-2] {
                return n3
            }
        }
        0
    }
}
