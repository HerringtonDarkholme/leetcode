impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, mut n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let total = mean * (m + n);
        let sum = rolls.into_iter().sum::<i32>();
        let mut sum = total - sum;
        if sum > 6 * n || sum < n {
            return vec![]
        }
        let mut ret = vec![];
        while n > 0 {
            let r = sum / n;
            ret.push(r);
            sum -= r;
            n -= 1;
        }
        ret
    }
}
