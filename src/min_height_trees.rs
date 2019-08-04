pub struct Solution;

use std::collections::VecDeque;
// key point:
// 1. we count all edges and push all leaf nodes to a vec!
// 2. for all nodes in leaf vecs, pop them and delete the node and edge from tree
// 3. for the new tree, list all leaf nodes to vec
// 4. MHT will have at most 2 nodes, if leaf nodes are fewer than 2, return
//   | first, prove all MHT roots are in the same longest path, otherwise suppose root1 path has longest
//   path h, and root2 path to root1 will have length h+1
//   | second, if MHT has three ort roots, the mid root on the same path  will have shorter path
//
// key optimization:
// since we delete only leaf nodes, exactly one edge will be removed, so we can represent edge in
// number, xor the number to toggle edge.

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0]
        }
        let n = n as usize;
        let mut degrees = vec![0; n];
        let mut adj_matrix = vec![0; n];
        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            degrees[i] += 1;
            degrees[j] += 1;
            adj_matrix[i] ^= j;
            adj_matrix[j] ^= i;
        }
        let mut leaves = VecDeque::new();
        for i in 0..degrees.len() {
            if degrees[i] == 1 {
                leaves.push_back(i);
            }
        }
        let mut m = n;
        while m > 2 {
            m -= leaves.len();
            for _ in 0..leaves.len() {
                let i = leaves.pop_front().unwrap();
                degrees[i] = 0;
                let j = adj_matrix[i];
                degrees[j] -= 1;
                if degrees[j] == 1 {
                    leaves.push_back(j);
                }
                adj_matrix[i] ^= j;
                adj_matrix[j] ^= i;
            }
        }
        leaves.into_iter().map(|i| i as i32).collect()
    }
}

#[test]
fn test() {
    for i in vec![
        (2, nested![[1,0]], vec![0, 1]),
        (3, nested![[1,0],[0,2]], vec![0]),
        (4, nested![[1,0],[1,2],[1,3]], vec![1]),
        (6, nested![[0, 3], [1, 3], [2, 3], [4, 3], [5, 4]], vec![3, 4]),
    ] {
        assert_eq!(Solution::find_min_height_trees(i.0, i.1), i.2);
    }
}
