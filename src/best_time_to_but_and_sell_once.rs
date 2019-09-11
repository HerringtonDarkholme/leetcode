pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::min_value();
        let mut sell = 0;
        for p in prices {
            sell = sell.max(buy + p);
            buy = buy.max(-p);
        }
        sell
    }
}
