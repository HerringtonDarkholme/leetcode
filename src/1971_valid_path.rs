impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination { return true; }
        let map = build_map(n, edges);
        let mut visited = vec![false; n as usize];
        dfs(&map, source as usize, destination as usize, &mut visited)
    }
}
fn build_map(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut map = vec![vec![]; n as usize];
    for edge in edges {
        let (s, e) = (edge[0] as usize, edge[1] as usize);
        map[s].push(e);
        map[e].push(s);
    }
    map
}
fn dfs(map: &Vec<Vec<usize>>, s: usize, d: usize, visited: &mut Vec<bool>) -> bool {
    if visited[s] { return false; }
    visited[s] = true;
    for &next in &map[s] {
        if next == d { return true; }
        if dfs(map, next, d, visited) {
            return true;
        }
    }
    false
}
