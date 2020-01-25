use std::collections::{
    HashSet,
};

impl Solution {
    pub fn max_candies(mut status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        for &b in initial_boxes.iter() {
            status[b as usize] |= 0b10; // box in hand
        }
        let mut ready_boxes: Vec<usize> = initial_boxes
            .iter().cloned()
            .map(|i| i as usize)
            .filter(|&i| status[i] == 0b11)
            .collect();
        let mut ret = 0;
        while !ready_boxes.is_empty() {
            let opened = ready_boxes.pop().unwrap();
            status[opened] = 0b111;
            ret += candies[opened];
            for &k in keys[opened].iter() {
                let k = k as usize;
                if status[k] == 0b10 {
                    ready_boxes.push(k);
                }
                status[k] |= 0b01; // open box
            }
            for &b in contained_boxes[opened].iter() {
                let b = b as usize;
                status[b] |= 0b10;
                if status[b] == 0b11 { //opened
                    ready_boxes.push(b);
                }
            }
        }
        ret
    }
}
