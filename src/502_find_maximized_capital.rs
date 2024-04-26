use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects = vec![];
        for i in 0..profits.len() {
            projects.push((capital[i], profits[i]));
        }
        projects.sort();
        projects.reverse();
        let mut heap = BinaryHeap::new();
        for _ in 0..k {
            while let Some(&(cap, pro)) = projects.last() {
                if w < cap { break; }
                projects.pop();
                heap.push(pro);
            }
            if let Some(p) = heap.pop() {
                w += p;
            } else {
                break;
            }
        }
        w
    }
}
