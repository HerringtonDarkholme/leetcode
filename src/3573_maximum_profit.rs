impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let mut long = vec![i64::MIN; k as usize + 1];
        let mut short = vec![0; k as usize + 1];
        let mut neutral = vec![0; k as usize + 1];
        for price in prices {
            let mut next_l = vec![i64::MIN; k as usize + 1];
            let mut next_s = vec![0; k as usize + 1];
            let mut next_n = vec![0; k as usize + 1];
            let price = price as i64;
            for i in 0..=(k as usize) {
                next_l[i] = (neutral[i] - price).max(long[i]);
                next_s[i] = (neutral[i] + price).max(short[i]);
                if i != 0 {
                    next_n[i] = (long[i - 1] + price).max(short[i - 1] - price).max(neutral[i]);
                }
            }
            long = next_l;
            short = next_s;
            neutral = next_n;
        }
        neutral.into_iter().max().unwrap()
    }
}
