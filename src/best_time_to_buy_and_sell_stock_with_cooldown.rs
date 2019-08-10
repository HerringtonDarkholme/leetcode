pub struct Solution;

// To sovle this problem with DP, we can give 2 definition :
// 1.buy[i], means the max profit we can get if the status end with buy([buy,cooldown,cooldown] also means end with buy) at i-th day(i=0,1,2...)
// 2.sell[i], means the max profit we can get if the status end with sell([sell,cooldown,cooldown] also means end with sell) at i-th day(i=0,1,2...)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0
        }
        if prices.len() == 2 {
            return 0.max(prices[1] - prices[0])
        }
        let mut buy = vec![0; prices.len()];
        let mut sell = vec![0; prices.len()];
        buy[0] = -prices[0]; // buy on day1, we have negative asset
        buy[1] = buy[0].max(-prices[1]); // hold on: asset = buy[0], or buy 1-st day stock
        sell[0] =  0; // nothing sell on day1
        sell[1] = sell[0].max(prices[1] + buy[0]); // hold on, or sell on 1-st day
        for i in 2..prices.len() {
            // To calculate buy[i]:
            // 1.If we choose to buy at i-th day, then buy[i] = sell[i-2]-prices[i].
            // Because the i-1-th day must be cooldown, and we spend prices[i] to buy.
            // 2.If we choose to cooldown at i-th day, then buy[i] = buy[i-1].Notice this also means end with buy.
            // So buy[i] = max(sell[i-2]-prices[i],buy[i-1])
            buy[i] = buy[i - 1].max(sell[i - 2] - prices[i]);
            // To calculate sell[i]:
            // 1.If we choose to sell at i-th day, then sell[i] = buy[i-1]+prices[i].
            // 2.If we choose to cooldown at i-th day, then sell[i] = sell[i-1].
            // So sell[i] = max(buy[i-1]+prices[i],sell[i-1])
            sell[i] = sell[i - 1].max(buy[i-1] + prices[i]);
        }
        sell[prices.len() - 1]
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (vec![1, 2, 3, 0, 2], 3),
        (vec![1, 2, 3, 0, 5], 6),
        (vec![1, 1, 1, 1], 0),
        (vec![4, 3, 2, 1], 0),
        (vec![], 0),
    ], Solution::max_profit);
}
