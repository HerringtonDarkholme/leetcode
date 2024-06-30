impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
      height(red, blue).max(height(blue, red))
    }
}

fn height(mut odd: i32, mut even: i32) -> i32 {
  let mut n = 0;
  let mut height = 0;
  loop {
    if height % 2 == 0 {
      if even < n { break height - 1 }
      even -= n;
    } else {
      if odd < n { break height - 1 }
      odd -= n;
    }
    n += 1;
    height += 1;
  }
}

impl Solution {
  pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut all_even = 0;
    let mut all_odd = 0;
    let mut alt_even = 0;
    let mut alt_odd = 0;
    for num in nums {
      if num % 2 == 0 {
        all_even += 1;
        alt_odd = alt_odd.max(alt_even + 1);
      } else {
        all_odd += 1;
        alt_even = alt_even.max(alt_odd + 1);
      }
    }
    all_odd.max(all_even).max(alt_even).max(alt_odd)
  }
}

// impl Solution {
//   pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
//     let mut lenss = vec![vec![0; k as usize]; k as usize];
//     for num in nums {
//       let i = (num % k) as usize;
//       for sum in 0..lenss.len() {
//         let mut lens = &mut lenss[sum];
//         let other = (k as usize + sum - i) % (k as usize);
//         lens[other] = lens[other].max(lens[i] + 1);
//       }
//     }
//     let mut ret = 0;
//     for lens in lenss {
//       for len in lens {
//         ret = ret.max(len);
//       }
//     }
//     ret
//   }
// }

impl Solution {
  pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let g1 = graph(edges1);
    let g2 = graph(edges2);
    let mut max = 0;
    let s1 = find_span(g1, &mut max);
    let s2 = find_span(g2, &mut max);
    (s1 + s2 + 1).max(max) as i32
  }
}

use std::collections::{HashSet, VecDeque};
fn graph(edges: Vec<Vec<i32>>) -> Vec<HashSet<usize>> {
  let mut g = vec![HashSet::new(); edges.len() + 1];
  for edge in edges {
    let i = edge[0] as usize;
    let j = edge[1] as usize;
    g[i].insert(j);
    g[j].insert(i);
  }
  g
}

fn find_span(mut g: Vec<HashSet<usize>>, span: &mut usize) -> usize {
  let mut dis = vec![0; g.len()];
  let mut leaves = {
    let mut frontier = VecDeque::new();
    for i in 0..g.len() {
      if g[i].len() == 1 {
        frontier.push_back(i);
      }
    }
    frontier
  };
  while let Some(leaf) = leaves.pop_front() {
    let Some(other) = g[leaf].drain().next() else { break; };
    g[other].remove(&leaf);
    let max = dis[other] + dis[leaf] + 1;
    *span = max.max(*span);
    dis[other] = dis[other].max(dis[leaf] + 1);
    if g[other].len() == 1 {
      leaves.push_back(other);
    }
  }
  dis.into_iter().max().unwrap()
}
