impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrive: Vec<_> = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(d, s)| (d/s, d%s))
            .collect();
        arrive.sort();
        for i in 0..arrive.len() as i32 {
            let (t, r) = arrive[i as usize];
            if t < i || (t == i && r == 0) {
                return i;
            }
        }
        arrive.len() as i32
    }
}
