pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let dist = |p: &Vec<i32>| {p[0] * p[0] + p[1] * p[1]};
        let mut p = points.iter()
            .enumerate()
            .map(|(k, p)| (k, dist(p)))
            .collect::<Vec<_>>();
        p.sort_by_key(|i| i.1);
        p.into_iter().take(k as usize).map(|t| points[t.0].clone()).collect()

        // points.into_iter().take(k as usize).collect()
    }
}
