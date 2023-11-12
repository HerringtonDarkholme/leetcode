use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target { return 0 }
        let stops = build_stops(routes);
        let target_routes: HashSet<_> = match stops.get(&target) {
            Some(v) => v.iter().cloned().collect(),
            None => return -1,
        };
        let mut routes = match stops.get(&source) {
            Some(v) => v.clone(),
            None => return -1,
        };
        let connections = build_connections(&stops);
        let mut visited = HashSet::new();
        let mut buses = 0;
        while !routes.is_empty() {
            let mut next = vec![];
            buses += 1;
            for route in routes {
                if target_routes.contains(&route) {
                    return buses;
                }
                visited.insert(route);
                // build next
                for &connected in &connections[route] {
                    if visited.contains(&connected) {
                        continue;
                    }
                    next.push(connected);
                }
            }
            routes = next;
        }
        -1 
    }
}
// stop -> routes
fn build_stops(routes: Vec<Vec<i32>>) -> HashMap<i32, Vec<usize>> {
    let mut map = HashMap::new();
    for (route, stops) in routes.into_iter().enumerate() {
        for stop in stops {
            map.entry(stop).or_insert_with(Vec::new).push(route);
        }
    }
    map
}
// route -> connected routes
fn build_connections(stops: &HashMap<i32, Vec<usize>>) -> Vec<Vec<usize>> {
    let mut map = HashMap::new();
    for routes in stops.values() {
        for &route in routes {
            let mut connected = map.entry(route).or_insert_with(HashSet::new);
            for &r in routes {
                if r != route {
                    connected.insert(r);
                }
            }
        }
    }
    let mut ret = vec![vec![]; 500];
    for (r, connected) in map {
        ret[r] = connected.into_iter().collect();
    }
    ret
}



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
