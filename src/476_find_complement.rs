impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        while num != 0 {
            ret |= ((num ^ 1) & 1) << i;
            i += 1;
            num >>= 1;
        }
        ret
    }
}
