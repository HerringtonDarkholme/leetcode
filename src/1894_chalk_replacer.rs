impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut sums = vec![0; chalk.len()];
        let mut sum = 0i64;
        for (i, c) in chalk.into_iter().enumerate() {
            sum += c as i64;
            sums[i] = sum;
        }
        match sums.binary_search(&((k as i64) % sum)) {
            Ok(i) => i as i32 + 1,
            Err(i) => i as i32,
        }
    }
}
