impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<_> = points
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect();
        points.sort_by_key(|a| a.1);
        let mut pos = points[0].1;
        let mut ans = 1;
        for point in points {
            if point.0 > pos {
                ans += 1;
                pos = point.1;
            }
        }
        ans
    }
}
