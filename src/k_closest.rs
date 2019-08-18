pub struct Solution;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let dist = |p: &Vec<i32>| {p[0] * p[0] + p[1] * p[1]};
        points.sort_by_key(dist);
        points.into_iter().take(k as usize).collect()
    }
}
