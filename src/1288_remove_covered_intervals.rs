impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return intervals.len() as i32
        }
        intervals.sort_by_key(|v| (v[0], -v[1]));
        let mut last = &intervals[0];
        let mut count = 1;
        for i in 1..intervals.len() {
            let i = &intervals[i];
            if i[1] > last[1] {
                last = i;
                count += 1;
            }
        }
        count
    }
}
