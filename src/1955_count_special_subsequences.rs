const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let mut c = [0i64, 0i64, 0i64];
        for n in nums {
            let n = n as usize;
            if n == 0 {
                c[n] += 1 + c[0];
            } else if n == 1 {
                c[n] += c[0] + c[1];
            } else {
                c[n] += c[1] + c[2];
            }
            c[n] = c[n] % M;
        }
        c[2] as i32
    }
}
