use std::collections::HashMap;
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut found = false;
        let mut min = usize::MAX;
        for i in 0..cards.len() {
            if let Some(&j) = map.get(&cards[i]) {
                found = true;
                min = min.min(i - j);
            }
            map.insert(cards[i], i);
        }
        if found {
            min as i32 + 1
        } else {
            -1
        }
    }
}
