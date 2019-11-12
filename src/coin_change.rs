pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0
        }
        let amount = amount as usize;
        let mut coins: Vec<_> = coins.iter().map(|a| *a as usize).collect();
        coins.sort_by_key(|r| std::cmp::Reverse(*r));
        let mut cache = vec![0; amount + 1];
        aux(&coins, amount, &mut cache)
    }
}

fn aux(coins: &Vec<usize>, amount: usize, cache: &mut Vec<i32>) -> i32 {
    if cache[amount] != 0 {
        return cache[amount]
    }
    let mut min = -2;
    for &n in coins.iter() {
        if n == amount {
            cache[amount] = 1;
            return 1
        } else if n > amount {
            continue;
        } else {
            let s = aux(coins, amount - n, cache);
            if s != -1 {
                min = if min < 0 { s } else { min.min(s) };
            }
        }
    }
    cache[amount] = min + 1;
    min + 1
}
