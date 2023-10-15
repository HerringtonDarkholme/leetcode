use std::collections::HashSet;
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        let mut visited = HashSet::new();
        recurse(0, 0, jug1_capacity as usize, jug2_capacity as usize, target_capacity as usize, &mut visited)
    }
}

fn recurse(j1: usize, j2: usize, cap1: usize, cap2: usize, target: usize, visited: &mut HashSet<(i32, i32)>) -> bool {
    if j1 == target || j2 == target || j1 + j2 == target {
        return true;
    }
    if visited.contains(&(j1 as i32, j2 as i32)) {
        return false;
    }
    visited.insert((j1 as i32, j2 as i32));
    recurse(cap1, j2, cap1, cap2, target, visited) || // fill 1
    recurse(j1, cap2, cap1, cap2, target, visited) || // fill 2
    recurse(0, j2, cap1, cap2, target, visited) || // empty 1
    recurse(j1, 0, cap1, cap2, target, visited) || // empty 2
    recurse(
        if j1+j2 > cap2 { j1+j2-cap2 } else { 0 }, 
        cap2.min(j1 + j2), 
        cap1, cap2, target, visited) || // move 1 to 2 
    recurse(
        cap1.min(j1 + j2), 
        if j1 + j2 > cap1 { j1 + j2 - cap1 } else { 0 }, 
        cap1, cap2, target, visited) // move 2 to 1 
}
