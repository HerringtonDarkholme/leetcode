impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<_> = points.into_iter().map(|v| v[0]).collect();
        points.sort();
        let mut max = 0;
        for i in 1..points.len() {
            max = max.max(points[i] - points[i - 1]);
        }
        max
    }
}
