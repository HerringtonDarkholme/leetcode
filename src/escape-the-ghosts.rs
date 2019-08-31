pub struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let compute_distance = |t: &Vec<i32>, p: &Vec<i32>| (t[0] - p[0]).abs() + (t[1] - p[1]).abs();
        let td = compute_distance(&vec![0, 0], &target);
        ghosts.iter().all(|g| compute_distance(g, &target) > td)
    }
}
