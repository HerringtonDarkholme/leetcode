pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = a.len() - 2;
        while low < high {
            let mid = low + (high - low) / 2;
            if a[mid] < a[mid + 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}
