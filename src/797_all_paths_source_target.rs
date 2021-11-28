impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        dfs(0, &graph)
    }
}

fn dfs(start: usize, graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if start == graph.len() - 1 {
        return vec![vec![start as i32]];
    }
    let mut ret = vec![];
    for &i in graph[start].iter() {
        for mut v in dfs(i as usize, graph) {
            v.insert(0, start as i32);
            ret.push(v);
        }
    }
    ret
}
