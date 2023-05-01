impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let n = salary.len();
        for s in salary {
            sum += s;
            min = min.min(s);
            max = max.max(s);
        }
        (sum - min - max) as f64 / (n - 2) as f64
    }
}
