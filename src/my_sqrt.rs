pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x
        }
        let mut babylon = x / 2;
        let mut next = (babylon + x/babylon) / 2;
        while babylon != next && babylon != next - 1 {
            babylon = next;
            next = (babylon + x/babylon) / 2;
        }
        babylon
    }
}
