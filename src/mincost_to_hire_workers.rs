use std::collections::BinaryHeap;
use std::cmp::Ordering::Equal;

pub struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut unit = vec![];
        for i in 0..wage.len() {
            let p = (wage[i] as f64 / quality[i] as f64, i);
            unit.push(p);
        }
        unit.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal));
        let mut heap = BinaryHeap::new();
        let mut total_quality = 0;
        for i in 0..k as usize {
            let (u, idx) = unit[i];
            heap.push(quality[idx]);
            total_quality += quality[idx];
        }
        let mut cost = total_quality as f64 * unit[k as usize - 1].0;
        for i in (k as usize)..wage.len() {
            let (u, idx) = unit[i];
            let q = heap.pop().unwrap();
            total_quality -= q;
            heap.push(quality[idx]);
            total_quality += quality[idx];
            cost = cost.min(total_quality as f64 * u);
        }
        cost
    }
}
