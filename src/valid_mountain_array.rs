pub struct Solution;
impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 || a[0] >= a[1] {
            return false
        }
        let mut has_pivot = false;
        let mut prev = a[1];
        for n in 2..a.len() {
            if prev > a[n] {
                if !has_pivot {
                    has_pivot = true;
                }
            } else if prev < a[n] {
                if has_pivot {
                    return false
                }
            } else {
                return false
            }
            prev = a[n];
        }
        has_pivot
    }
}
