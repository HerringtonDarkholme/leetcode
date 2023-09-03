
pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize -1;
        let n = n as usize - 1;
        let mut ret = 1;
        let mut d = m+1;
        let mut k = 1;
        for i in 0..n {
            ret *= d;
            d += 1;
            if ret % k == 0 {
                ret /= k;
                k += 1;
            }
        }
        while k < n {
            ret /= k;
            k += 1;
        }
        ret as i32
    }
}

/*
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize -1;
        let n = n as usize - 1;
        let mut ret = 1;
        let mut d = m+1;
        let mut k = 1;
        for i in 0..n {
            ret *= d;
            d += 1;
            ret /= k;
            k += 1;
        }
        ret as i32
    }
}
 **/
