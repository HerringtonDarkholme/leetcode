pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();
        let mut deque = VecDeque::with_capacity(deck.len());
        deque.push_back(deck.pop().unwrap());
        while let Some(i) = deck.pop() {
            let j = deque.pop_back().unwrap();
            deque.push_front(j);
            deque.push_front(i);
        }
        deque.into_iter().collect()
    }
}
