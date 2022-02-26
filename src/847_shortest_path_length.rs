use std::collections::HashSet;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        if n == 1 {
            return 0
        }
        let (mut frontier, mut visited) = initial_nodes(n);
        let mut i = 0;
        while !frontier.is_empty() {
            let mut next = vec![];
            i += 1;
            for (node, state) in frontier {
                for &neighbor in &graph[node] {
                    let neighbor = neighbor as usize;
                    let new_state = state | (1 << neighbor);
                    if new_state.count_ones() == n as u32 {
                        return i
                    }
                    if has_visited(neighbor, new_state, &visited) {
                        continue;
                    }
                    visit(neighbor, new_state, &mut visited);
                    next.push((neighbor, new_state));
                }
            }
            frontier = next;
        }
        // impossible
        -1
    }
}

fn initial_nodes(n: usize) -> (Vec<(usize, u16)>, Vec<HashSet<u16>>) {
    let mut frontier = vec![];
    let mut visited = vec![HashSet::new(); n];
    for i in 0..n {
        let state = 1 << i;
        frontier.push((i, state));
        visited[i].insert(state);
    }
    (frontier, visited)
}

fn has_visited(n: usize, state: u16, visited: &Vec<HashSet<u16>>) -> bool {
    visited[n].contains(&state)
    // for &s in &visited[n] {
    //     if (state | s) == s {
    //         return true;
    //     }
    // }
    // false
}

fn visit(n: usize, state: u16, visited: &mut Vec<HashSet<u16>>) {
    // let mut next_visited = HashSet::new();
    // for &s in &visited[n] {
    //     if (state & s) != s {
    //         next_visited.insert(s);
    //     }
    // }
    visited[n].insert(state);
    // visited[n] = next_visited;
}
