use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let prefix = prefix_sum(piles);
        let mut map = HashMap::new();
        aux(0, 1, &prefix, &mut map)
    }
}

fn prefix_sum(piles: Vec<i32>) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(piles.len());
    prefix.push(0);
    let mut sum = 0;
    for p in piles {
        sum += p;
        prefix.push(sum);
    }
    prefix
}

fn aux(n: usize, m: usize, prefix: &[i32], dp: &mut HashMap<(usize, usize), i32>) -> i32 {
    if n >= prefix.len() {
        return 0;
    }
    if let Some(&ret) = dp.get(&(n, m)) {
        return ret;
    }
    let sum = prefix[prefix.len() - 1] - prefix[n];
    let mut max = 0;
    for i in 1..=(2 * m).min(prefix.len()) {
        max = max.max(sum - aux(n + i, m.max(i), prefix, dp));
    }
    dp.insert((n, m), max);
    max
}

// a(n, m) = max(sum - a(n + i, max(m, i)) for i in range(1, 2m))
