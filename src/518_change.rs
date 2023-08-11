use std::collections::HashMap;
impl Solution {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        // how many ways to get the amount
        let mut ways = HashMap::new();
        ways.insert(amount, 1);
        for coin in coins { // use different kinds of coins
            let mut next = HashMap::new();
            for (amt, way) in ways {
                for i in 0..=amt/coin {
                    *next.entry(amt - i * coin).or_insert(0) += way;
                }
            }
            ways = next;
        }
        *ways.get(&0).unwrap_or(&0)
    }
}
