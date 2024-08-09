#[macro_use]
mod util;
#[path = "./recursive_remove_adjacent_duplicates.rs"]
pub mod remove_adj;

fn main() {}

#[test]
fn test() {
    // assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
    // assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
}

struct Solution;

impl Solution {
  pub fn get_smallest_string(s: String) -> String {
    let mut ret = vec![];
    let mut switched = false;
    for c in s.chars() {
      if let Some(&p) = ret.last() {
        let is_same_parity = (p as u8 - b'0') % 2 == (c as u8 - b'0') % 2;
        if p > c && !switched && is_same_parity {
          ret.pop();
          ret.push(c);
          ret.push(p);
          switched = true;
          continue;
        }
      }
      ret.push(c);
    }
    ret.into_iter().collect()
  }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
use std::collections::HashSet;
impl Solution {
  pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let nums: HashSet<_> = nums.into_iter().collect();
    let mut ret = None;
    let mut h = &mut ret;
    while let Some(mut node) = head {
      head = node.next.take();
      if !nums.contains(&node.val) {
        *h = Some(node);
        h = &mut h.as_mut().unwrap().next;
      }
    }
    ret
  }
}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    let horizontal = prefix(horizontal_cut);
    let vertical = prefix(vertical_cut);
    let ret = helper(0, m as usize, 0, n as usize, &horizontal, &vertical, &mut dp);
    println!("{dp:?}");
    ret
  }
}

fn prefix(nums: Vec<i32>) -> Vec<i32> {
  let mut ret = vec![0];
  let mut sum = 0;
  for n in nums {
    sum += n;
    ret.push(sum);
  }
  ret
}

type DP = HashMap<(usize, usize, usize, usize), i32>;
// min cost = find min in horizontal_cut and vertical_cut
fn helper(sr: usize, er: usize, sc: usize, ec: usize, horizontal: &[i32], vertical: &[i32], dp: &mut DP) -> i32 {
  if let Some(&ret) = dp.get(&(sr, er, sc, ec)) {
    return ret;
  }
  let cost = if er == sr + 1 {
    vertical[ec - 1] - vertical[sc]
  } else if ec == sc + 1 {
    horizontal[er - 1] - horizontal[sr]
  } else {
    let mut min = i32::MAX;
    for i in sr+1..er {
      let m =
        (horizontal[i] - horizontal[i-1]) + helper(sr, i, sc, ec, horizontal, vertical, dp) + helper(i, er, sc, ec, horizontal, vertical, dp);
      min = min.min(m);
    }
    for i in sc+1..ec {
      let m =
        (vertical[i] -vertical[i - 1]) + helper(sr, er, sc, i, horizontal, vertical, dp) + helper(sr, er, i, ec, horizontal, vertical, dp);
      min = min.min(m);
    }
    min
  };
  dp.insert((sr, er, sc, ec), cost);
  cost
}
