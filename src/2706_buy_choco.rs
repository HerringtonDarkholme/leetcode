impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort();
        if prices[0] + prices[1] > money {
            money
        } else {
            money - prices[0] - prices[1]
        }
    }
}
