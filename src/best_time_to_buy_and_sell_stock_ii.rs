pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0
        }
        let mut hold = -prices[0];
        let mut free = 0;
        for p in prices {
            let o_hold = hold;
            hold = hold.max(free - p); // either keep hold or open transaction
            free = free.max(o_hold + p); // either keep free or close transaction
        }
        free
    }
}
