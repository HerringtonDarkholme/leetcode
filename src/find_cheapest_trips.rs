pub struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let graph = build_graph(n, flights);
        traverse(src as usize, dst as usize, k, graph)
    }
}

fn build_graph(n: usize, flights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut graph = vec![vec![0; n]; n];
    for flight in flights {
        let src = flight[0] as usize;
        let dst = flight[1] as usize;
        let price = flight[2];
        graph[src][dst] = price;
    }
    graph
}

fn traverse(src: usize, dst: usize, k: i32, graph: Vec<Vec<i32>>) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push((0, src, k + 1));
    while !heap.is_empty() {
        let (price, src, k) = heap.pop().unwrap();
        if src == dst {
            return -price
        }
        for i in 0..graph[src].len() {
            let next = graph[src][i];
            if next == 0 {
                continue;
            }
            if k == 0 {
                continue;
            }
            heap.push((price - next, i, k - 1));
        }
    }
    -1
}
