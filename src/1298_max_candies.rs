use std::collections::{
    HashSet,
};

impl Solution {
    pub fn max_candies(mut status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let mut ready_boxes: Vec<usize> = initial_boxes
            .iter().cloned()
            .map(|i| i as usize)
            .filter(|&i| status[i] == 1)
            .collect();
        let mut closed_boxes: HashSet<usize> = initial_boxes
            .iter().cloned()
            .map(|i| i as usize)
            .filter(|&i| status[i] == 0)
            .collect();
        let mut ret = 0;
        while !ready_boxes.is_empty() {
            let opened = ready_boxes.pop().unwrap();
            ret += candies[opened as usize];
            for &k in keys[opened].iter() {
                let k = k as usize;
                status[k] = 1;
                if closed_boxes.contains(&k) {
                    closed_boxes.remove(&k);
                    ready_boxes.push(k);
                }
            }
            for &b in contained_boxes[opened].iter() {
                let b = b as usize;
                if status[b] == 1 {
                    ready_boxes.push(b);
                } else {
                    closed_boxes.insert(b);
                }
            }
        }
        ret
    }
}
