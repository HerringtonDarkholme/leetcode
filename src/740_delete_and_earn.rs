use std::collections::HashMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut map: Vec<_> = map.into_iter().collect();
        if map.is_empty() {
            return 0;
        }
        map.sort();
        let mut taken = map[0].0 * map[0].1;
        let mut not_taken = 0;
        for i in 1..map.len() {
            let (n, cnt) = map[i];
            let prev_n = map[i - 1].0;
            let (n_taken, n_not_taken) = if prev_n + 1 == n {
                (not_taken + n * cnt, taken.max(not_taken))
            } else {
                let m = taken.max(not_taken);
                (n * cnt + m, m)
            };
            taken = n_taken;
            not_taken = n_not_taken;
        }
        taken.max(not_taken)
    }
}

// 1 n1 4 n2 5n3 6
// max(i) = max(i_taken) max max(i_not_taken)
// max(i_taken) =  earn[i] + max(i-1_not_taken)
// max(i_not_taken) = max(i-1_not_taken) max max(taken)
