use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, mut values: Vec<i32>, k: i32) -> i32 {
        if edges.is_empty() { return 1 }
        let mut matrix = build_matrix(edges);
        let mut leaves = find_leaves(&matrix);
        let mut ret = 0;
        while let Some(leaf) = leaves.pop() {
            let neighbor = cut(leaf, &mut matrix, &mut leaves); // remove 
            if values[leaf as usize] % k == 0 {
                ret += 1; // found cut
            } else { // merge neighbor
                values[neighbor as usize] += values[leaf as usize];
                values[neighbor as usize] %= k;
            }
        }
        ret
    }
}
fn build_matrix(edges: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut matrix = HashMap::new();
    for edge in edges {
        let n1 = edge[0];
        let n2 = edge[1];
        matrix.entry(n1).or_insert_with(HashSet::new).insert(n2);
        matrix.entry(n2).or_insert_with(HashSet::new).insert(n1);
    }
    matrix
}
fn find_leaves(matrix: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    matrix.iter().filter_map(|(k, v)| {
        if v.len() == 1 { Some(*k) } else { None }
    }).collect()
}
fn remove_leaf_and_get_neighbor(leaf: i32, matrix: &mut HashMap<i32, HashSet<i32>>) -> i32 {
    let neighbors = matrix.remove(&leaf).unwrap();
    debug_assert_eq!(neighbors.len(), 1);
    // -1 only when there is only one node left
    neighbors.into_iter().next().unwrap_or(-1)
} 
fn cut(leaf: i32, matrix: &mut HashMap<i32, HashSet<i32>>, leaves: &mut Vec<i32>) -> i32 {
    let neighbor = remove_leaf_and_get_neighbor(leaf, matrix);
    if neighbor == -1 { return -1 }
    let next = matrix.get_mut(&neighbor).unwrap();
    next.remove(&leaf);
    if next.len() == 1 {
        leaves.push(neighbor);
    }
    neighbor
}
