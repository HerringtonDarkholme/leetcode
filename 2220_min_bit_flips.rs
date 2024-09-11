impl Solution {
    pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
        let mut ret = 0;
        while start > 0 {
            ret += (start & 1) ^ (goal & 1);
            start >>= 1;
            goal >>= 1;
        }
        ret += goal.count_ones() as i32;
        ret
    }
}
