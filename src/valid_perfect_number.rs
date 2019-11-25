pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut low = 1;
        let mut high = 46340.min(num / 2);
        while low < high {
            let mid = low + (high - low) / 2;
            if mid * mid < num {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low * low == num
    }
}
