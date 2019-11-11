// Leetcod 815
pub struct Solution;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
        if s == t {
            return 0
        }
        let rnum = routes.len();
        let mut graph = vec![vec![false; rnum]; rnum];
        let mut map = HashMap::new();
        for i in 0..rnum {
            let route = &routes[i];
            for &s in route.iter() {
                map.entry(s).or_insert(vec![]).push(i);
            }
        }
        for shared in map.values() {
            for i in 0..shared.len() {
                for j in i..shared.len() {
                    let ii = shared[i];
                    let jj = shared[j];
                    graph[ii][jj] = true;
                    graph[jj][ii] = true;
                }
            }
        }
        let ends: HashSet<_> = map[&t].iter().cloned().collect();
        let mut visited = vec![false; rnum];
        let mut queue = VecDeque::new();
        for &start in map[&s].iter() {
            queue.push_back((start, 1));
            visited[start] = true;
        }
        while !queue.is_empty() {
            let (route, num) = queue.pop_front().unwrap();
            if ends.contains(&route) {
                return num
            }
            for (other, &connected) in graph[route].iter().enumerate() {
                if !connected || visited[other] {
                    continue;
                }
                visited[other] = true;
                queue.push_back((other, num+1));
            }
        }
        -1
    }
}
