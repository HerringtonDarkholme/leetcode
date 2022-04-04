use std::collections::HashSet;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut no_lose = HashSet::new();
        let mut lose_once = HashSet::new();
        let mut others = HashSet::new();
        for m in matches {
            let w = m[0];
            let l = m[1];
            if !others.contains(&w) && !lose_once.contains(&w) {
                no_lose.insert(w);
            }
            if others.contains(&l) {
                continue;
            }
            if lose_once.contains(&l) {
                lose_once.remove(&l);
                others.insert(l);
                continue;
            }
            if no_lose.contains(&l) {
                no_lose.remove(&l);
            }
            lose_once.insert(l);
        }
        let mut no_lose: Vec<_> = no_lose.into_iter().collect();
        let mut lose_once: Vec<_> = lose_once.into_iter().collect();
        no_lose.sort();
        lose_once.sort();
        vec![no_lose, lose_once]
    }
}

