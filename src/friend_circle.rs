pub struct Solution;

fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, start: usize) {
    if visited[start] {
        return;
    }
    visited[start] = true;
    for (j, &friend) in graph[start].iter().enumerate() {
        if friend == 1 {
            dfs(graph, visited, j);
        }
    }
}

impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; m.len()];
        let mut ret = 0;
        for i in 0..m.len() {
            if visited[i] {
                continue;
            }
            dfs(&m, &mut visited, i);
            ret += 1;
        }
        ret
    }
}
