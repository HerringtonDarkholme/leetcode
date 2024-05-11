use std::collections::BinaryHeap;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut included = BinaryHeap::new();
        let mut candidates: Vec<_> = (0..quality.len()).collect();
        // max efficiency -> min efficiency
        candidates.sort_unstable_by(|&a, &b| {
            (wage[b] * quality[a]).cmp(&(wage[a] * quality[b]))
        });
        let mut max_efficiency = 0.0;
        let mut total_quality = 0.0;
        for _ in 0..k { // build initial included
            let i = candidates.pop().unwrap();
            max_efficiency = wage[i] as f64 / quality[i] as f64;
            included.push(quality[i]);
            total_quality += quality[i] as f64;
        }
        let mut min = max_efficiency * total_quality;
        while let Some(i) = candidates.pop() {
            max_efficiency = wage[i] as f64 / quality[i] as f64;
            included.push(quality[i]);
            let removed = included.pop().unwrap();
            total_quality += (quality[i] - removed) as f64;
            min = min.min(max_efficiency * total_quality);
        }
        min
    }
}

/*
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
        let mut cost = -1.0f64;
        for (u, idx) in unit {
            if heap.len() >= k as usize {
                let q = heap.pop().unwrap();
                total_quality -= q;
            }
            total_quality += quality[idx];
            heap.push(quality[idx]);
            if heap.len() == k as usize {
                cost = if cost > 0.0 {
                    cost.min(total_quality as f64 * u)
                } else {
                    total_quality as f64 * u
                };

            }
        }
        cost
    }
}
*/
