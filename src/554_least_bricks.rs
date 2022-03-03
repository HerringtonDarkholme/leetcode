use std::collections::HashMap;
impl Solution {
    pub fn least_bricks(walls: Vec<Vec<i32>>) -> i32 {
        let mut counter = HashMap::new();
        let len = walls.len() as i32;
        for wall in walls {
            let mut w = 0;
            for i in (0..wall.len()-1) {
                w += wall[i];
                *counter.entry(w).or_insert(0) += 1;
            }
        }
        len - counter.into_values().max().unwrap_or(0)
    }
}

