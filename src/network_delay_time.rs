pub struct Solution;

use std::collections::{BinaryHeap};

// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
// struct Edge {
//     weight: i32,
//     to: usize,
// }

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for t in times {
            let from = t[0] as usize - 1;
            let to = t[1] as usize - 1;
            let weight = -t[2];
            graph[from].push((weight, to));
        }
        let mut queue = BinaryHeap::new();
        queue.push((0, k as usize - 1));
        let mut visited = vec![false; n];
        while let Some((cost, k)) = queue.pop() {
            visited[k] = true;
            if visited.iter().all(|&k| k) {
                return -cost
            }
            for e in graph[k].iter() {
                let to = e.1;
                if !visited[to] {
                    queue.push((e.0 + cost, e.1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    for i in vec![
        (nested![[2,1,1],[2,3,1],[3,4,1]], 4, 2, 2),
        (nested![[2,1,1],[2,3,1],[3,4,1]], 4, 3, -1),
        (nested![[1,2,1],[2,3,2],[1,3,2]], 3, 1, 2),
    ] {
        assert_eq!(Solution::network_delay_time(i.0, i.1, i.2), i.3);
    }
}
