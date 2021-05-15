const M: i32 = 1_000_000_007;

impl Solution {
    pub fn sum_of_floored_pairs(mut nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut count = vec![0; m+1];
        for n in nums {
            let n = n as usize;
            count[n] += 1;
        }
        let mut sum = vec![0; m+1];
        for i in 1..m+1 {
            sum[i] = sum[i - 1] + count[i];
        }
        let mut r = 0;
        for i in 1..m+1 {
            if count[i] <= 0 {
                continue;
            }
            let mut b = i;
            while b <= m {
                let upper = (b + i - 1).min(m);
                let s = (b/i) as i32 * (sum[upper] - sum[b-1]) * count[i] % M;
                r += s;
                r %= M;
                b += i;
            }
        }
        r
    }
}
