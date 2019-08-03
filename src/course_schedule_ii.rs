pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_order(num_courses: i32, pre: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();
        let num_courses = num_courses as usize;
        for p in pre {
            graph.entry(p[0]).or_insert_with(|| vec![])
                .push(p[1]);
        }
        // dbg!(&graph);
        let mut path = Vec::with_capacity(num_courses);
        let mut visited = vec![false; num_courses];
        for i in 0..num_courses {
            if !Solution::dfs(i as i32, &mut visited, &mut graph, &mut path) {
                return vec![]
            }
        }
        path
    }

    fn dfs(num: i32, visited: &mut [bool], graph: &mut HashMap<i32, Vec<i32>>, path: &mut Vec<i32>) -> bool {
        if path.contains(&num) {
            return true
        }
        if let Some(dep) = graph.remove(&num) {
            if visited[num as usize] {
                return false
            }
            visited[num as usize] = true;
            for &d in dep.iter() {
                if !Solution::dfs(d, visited, graph, path) {
                    return false
                }
            }
            path.push(num);
            visited[num as usize] = false;
        } else {
            path.push(num);
        }
        !visited[num as usize]
    }
}

#[test]
fn test() {
    for i in vec![
        (2, nested![[1, 0]], vec![0, 1]),
        (2, nested![[1, 0], [0, 1]], vec![]),
        (5, nested![[1,0], [2,4], [3,2], [4,1]], vec![0, 1, 4, 2, 3]),
        (5, nested![[1,0], [2,4], [3,2], [4,1], [1, 2]], vec![]),
    ] {
        assert_eq!(Solution::find_order(i.0, i.1), i.2);
    }
}
