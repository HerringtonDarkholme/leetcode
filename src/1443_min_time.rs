impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let children = build_children(n as usize, edges);
        let mut visited = vec![false; n as usize];
        min_time(0, &children, &has_apple, &mut visited).max(2) - 2
    }
}
fn build_children(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut children = vec![vec![]; n];
    for edge in edges {
        children[edge[0] as usize].push(edge[1] as usize);
        children[edge[1] as usize].push(edge[0] as usize);
    }
    children
}

fn min_time(n: usize, children: &Vec<Vec<usize>>, has_apple: &[bool], visited: &mut Vec<bool>) -> i32 {
    if visited[n] {
        return 0
    }
    visited[n] = true;
    let children_time: i32 =
        children[n]
            .iter()
            .map(|&child| min_time(child, children, has_apple, visited))
            .sum();
    if has_apple[n] {
        children_time + 2
    } else {
        if children_time > 0 { children_time + 2 } else { 0 }
    }
}
