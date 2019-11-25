pub struct Solution;

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut min = i32::max_value();
        let mut max = 0;
        let mut mode = 0;
        let mut sum = 0usize;
        let mut cnt = 0;
        let total = count.iter().sum::<i32>();
        let mut median = 0.0;
        for n in 0..count.len() {
            let c = count[n];
            sum += n * c as usize;
            if c > 0 {
                max = n;
                if min == i32::max_value() {
                    min = n as i32;
                }
            }
            if c > count[mode] {
                mode = n;
            }
            if cnt < total/2 && cnt + c > total/2 {
                median = n as f64;
            } else if cnt == total / 2 {
                median = if total % 2 == 0 {
                    n as f64 - 0.5
                } else {
                    n as f64
                };
            }
            cnt += c;
        }
        vec![
            min as f64,
            max as f64,
            (sum as f64) / (total as f64),
            median,
            mode as f64
        ]
    }
}
