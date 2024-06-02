use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn clear_stars(s: String) -> String {
      let mut heap = BinaryHeap::new();
      for (i, c) in s.chars().enumerate() {
        if c == '*' {
          heap.pop();
        } else {
          heap.push((Reverse(c), i));
        }
      }
      let mut ret = heap.into_vec();
      ret.sort_by_key(|n| n.1);
      ret.into_iter().map(|n| n.0.0).collect()
    }
}
