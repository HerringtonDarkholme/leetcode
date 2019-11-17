pub struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if citations.is_empty() {
            return 0
        }
        let mut low = 1;
        let len = citations.len();
        let mut high = len;
        while low < high {
            let mid = low + (high - low) / 2;
            let elem = citations[len - mid];
            if elem > mid as i32 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if citations[len - low] >= low as i32 {
            low as i32
        } else {
            low as i32 - 1
        }
    }
}
