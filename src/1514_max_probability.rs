use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
#[derive(PartialEq, Clone, Debug)]
struct NonNan(f64);
impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Eq for NonNan {}
impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let map = build_map(n as usize, edges, succ_prob);
        let mut heap: BinaryHeap<_> = BinaryHeap::new();
        heap.push((NonNan(1.0), start_node as usize));
        let mut visited = HashSet::new();
        while let Some((prob, node)) = heap.pop() {
            if node == end_node as usize {
                return prob.0;
            }
            if visited.contains(&node) { continue; }
            visited.insert(node);
            for (next, p) in &map[node] {
                heap.push((NonNan(prob.0 * p.0), *next));
            }
        }
        0.0
    }
}
fn build_map(n: usize, edges: Vec<Vec<i32>>, probs: Vec<f64>) -> Vec<Vec<(usize, NonNan)>> {
    let mut map = vec![vec![]; n];
    for (edge, prob) in edges.into_iter().zip(probs.into_iter()) {
        let (s, e) = (edge[0] as usize, edge[1] as usize);
        map[s].push((e, NonNan(prob)));
        map[e].push((s, NonNan(prob)));
    }
    map
}
