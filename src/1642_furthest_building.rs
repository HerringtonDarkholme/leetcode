use std::collections::BinaryHeap;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut greatest = BinaryHeap::new();
        let mut used_bricks = 0;
        for (i, &height) in heights.iter().enumerate().skip(1) {
            let diff = height - heights[i - 1];
            if diff <= 0 {
                continue;
            }
            greatest.push(-diff);
            if greatest.len() <= ladders as usize {
                continue;
            }
            let d = -greatest.pop().unwrap();
            used_bricks += d;
            if used_bricks > bricks {
                return i as i32 - 1;
            }
        }
        heights.len() as i32 - 1
    }
}

// for each diff, we can either use ladder or brick
// to minimize brick use, we can use ladder to fill the greatest height diff
// so it means to find greatest K elements in a sequence.
