pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_finish(num_courses: i32, pre: Vec<Vec<i32>>) -> bool {
        let mut graph = HashMap::new();
        let num_courses = num_courses as usize;
        for p in pre {
            graph.entry(p[0]).or_insert_with(|| vec![])
                .push(p[1]);
        }
        // dbg!(&graph);
        // let mut path = Vec::with_capacity(num_courses);
        let mut visited = vec![false; num_courses];
        for i in 0..num_courses {
            if !Solution::dfs(i as i32, &mut visited, &mut graph, ) {
                return false
            }
        }
        true
    }

    fn dfs(num: i32, visited: &mut [bool], graph: &mut HashMap<i32, Vec<i32>>) -> bool {
        if let Some(dep) = graph.remove(&num) {
            if visited[num as usize] {
                return false
            }
            visited[num as usize] = true;
            for &d in dep.iter() {
                if !Solution::dfs(d, visited, graph) {
                    return false
                }
            }
            visited[num as usize] = false;
        }
        !visited[num as usize]
    }
}

#[test]
fn test() {
    for i in vec![
        (2, nested![[1, 0]], true),
        (2, nested![[1, 0], [0, 1]], false),
        (5, nested![[1,0], [2,4], [3,2], [4,1]], true),
        (5, nested![[1,0], [2,4], [3,2], [4,1], [1, 2]], false),
    ] {
        assert_eq!(Solution::can_finish(i.0, i.1), i.2);
    }
}
