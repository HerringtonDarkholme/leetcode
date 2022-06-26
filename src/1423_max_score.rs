impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = card_points.iter().sum();
        let n = card_points.len() - k as usize;
        if n == 0 {
            return sum
        }
        let mut min: i32 = card_points[..n].iter().sum();
        let mut curr = min;
        for i in n..card_points.len() {
            curr = curr - card_points[i - n] + card_points[i];
            min = min.min(curr);
        }
        sum - min
    }
}
