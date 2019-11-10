pub struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; n + 1];
        cache[0] = 1;
        cache[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                cache[i] += cache[j - 1] * cache[i - j];
            }
        }
        cache[n]
    }
}
