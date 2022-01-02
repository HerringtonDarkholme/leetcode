// leetcode 1010
pub struct Solution;
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        const S: usize = 60;
        let mut counter = vec![0; S];
        let time = time.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let mut ret = 0;
        for t in time {
            ret += counter[(S - t % S) % S];
            counter[t % S] += 1;
        }
        ret
    }
}
