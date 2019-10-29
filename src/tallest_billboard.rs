// this is a transformded knacksack problem
pub struct Solution;

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut cache = vec![vec![None; 10001]; rods.len()];
        dp(0, 5000, &mut cache, &rods)
    }
}
// s is how many rod diff between left/right
fn dp(i: usize, s: usize, cache: &mut Vec<Vec<Option<i32>>>, rods: &Vec<i32>) -> i32 {
    if i == rods.len() {
        if s == 5000 {
            return 0
        } else {
            return i32::min_value()
        }
    }
    if let Some(&r) = cache[i][s].as_ref() {
        return r
    }
    let mut max = dp(i + 1, s, cache, rods);
    max = max.max(dp(i + 1, s - rods[i] as usize, cache, rods));
    max = max.max(dp(i + 1, s + rods[i] as usize, cache, rods) + rods[i]);
    cache[i][s] = Some(max);
    max
}
