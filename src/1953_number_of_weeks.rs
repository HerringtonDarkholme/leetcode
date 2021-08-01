impl Solution {
    pub fn number_of_weeks(mut milestones: Vec<i32>) -> i64 {
        milestones.sort();
        let s = milestones.iter().map(|a| *a as i64).sum();
        let max = milestones[milestones.len() - 1] as i64;
        if max * 2 > s {
            (s - max) * 2 + 1
        } else {
            s
        }
    }
}
