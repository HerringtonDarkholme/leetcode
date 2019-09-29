impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let len = graph.len();
        let mut part = vec![0; len];
        for i in 0..len {
            if part[i] == 0 { // not assigned
                // dfs ensure unassigned node is not connected to previous connected nodes
                if !dfs(&graph, &mut part, i, 1) { // assign i as 1
                    return false
                }
            }
        }
        true
    }
}

// we only call dfs on unassigned node
fn dfs(graph: &Vec<Vec<i32>>, part: &mut Vec<i32>, start: usize, p: i32) -> bool {
    let edges = &graph[start];
    part[start] = p;
    for &e in edges.iter() {
        let e = e as usize;
        if part[e] == 0 { // adjacent node not assigned
            if !dfs(graph, part, e, -p) { // assign to another part
                return false
            }
        } else if part[e] != -p { // conflict assignemnt
            return false
        }
    }
    true
}
