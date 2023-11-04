impl Solution {
    pub fn get_last_moment(n: i32, mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
        left.sort();
        right.sort();
        let l_time = if !left.is_empty() {
            left[left.len() - 1]
        } else { 0 };
        let r_time = if !right.is_empty() {
            n - right[0]
        } else { 0 };
        r_time.max(l_time)
    }
}
