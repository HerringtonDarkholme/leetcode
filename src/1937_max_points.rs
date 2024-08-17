impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut val: Vec<_> = points[0].iter().map(|c| *c as i64).collect();
        for row in points.into_iter().skip(1) {
            let mut next = vec![0; val.len()];
            let mut left_max = vec![0; val.len()];
            let mut right_max = vec![0; val.len()];
            left_max[0] = val[0] as i64;
            for i in 1..val.len() {
                left_max[i] = val[i].max(left_max[i - 1] - 1);
            }
            right_max[val.len() - 1] = val[val.len() - 1] as i64;
            for i in (0..(val.len() - 1)).rev() {
                right_max[i] = val[i].max(right_max[i + 1] - 1);
            }
            val = row.into_iter().enumerate().map(|(i, c)| {
                c as i64 + left_max[i].max(right_max[i]) as i64
            }).collect();
        }
        val.into_iter().max().unwrap()
    }
}
