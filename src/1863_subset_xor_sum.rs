impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut rets: Vec<i32> = vec![];
        let mut sum = 0;
        for n in nums {
            sum += n;
            let mut v = vec![n];
            v.extend(rets.iter().map(|&m| {
                sum += m ^ n;
                m ^ n
            }));
            rets.extend(v.iter());
        }
        sum
    }
}
