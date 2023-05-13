const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let (low, high, zero, one) = (low as usize, high as usize, zero as usize, one as usize);
        let mut v = vec![0; high + 1];
        v[0] = 1;
        let mut sum = 0;
        for i in 0..=high {
            if (low..=high).contains(&i) {
                sum += v[i];
                sum %= M;
            }
            if i + zero <= high {
                v[i + zero] += v[i];
                v[i + zero] %= M;
            }
            if i + one <= high {
                v[i + one] += v[i];
                v[i + one] %= M;
            }
        }
        sum as i32
    }
}
