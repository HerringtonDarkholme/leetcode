impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        if n == 0 {
            return 1
        }
        let mut ret = 0;
        let mut i = 0; 
        while n != 0 {
            ret |= ((n ^ 1) & 1) << i;
            i += 1;
            n >>= 1;
        }
        ret
    }
}
