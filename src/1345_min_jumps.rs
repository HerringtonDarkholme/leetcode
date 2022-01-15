use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map = build_map(&arr);
        let mut step = 0;
        let mut frontier = vec![arr.len() - 1];
        let mut visited = HashSet::new();
        visited.insert(arr.len() - 1);
        visited.insert(arr.len()); // insert one to avoid check i + 1 == len
        loop {
            let mut next = vec![];
            for i in frontier {
                if i == 0 {
                    return step;
                }
                if !visited.contains(&(i -1)) {
                    next.push(i - 1);
                    visited.insert(i - 1);
                }
                if !visited.contains(&(i + 1)) {
                    next.push(i + 1);
                    visited.insert(i + 1);
                }
                for &j in map[&arr[i]].iter() {
                    if i != j && !visited.contains(&j) {
                        next.push(j);
                        visited.insert(j);
                    }
                }
                map.insert(arr[i], vec![]); // don't visit arr[i] again
            }
            frontier = next;
            step += 1
        }
        step
    }
}

fn build_map(arr: &[i32]) -> HashMap<i32, Vec<usize>> {
    let mut map = HashMap::new();
    let len = arr.len();
    for (i, &n) in arr.iter().enumerate() {
        let entry = map.entry(n).or_insert_with(|| vec![]);
        entry.push(i);
    }
    map
}
