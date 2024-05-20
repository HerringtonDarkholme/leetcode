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
/*
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 32];
        count[0] = 1;
        let mut ret = 0usize;
        for num in nums.into_iter().map(|n| n as usize) {
            let mut next = count.clone();
            for (i, cnt) in count.into_iter().enumerate() {
                ret += (i ^ num) * cnt;
                next[i ^ num] += cnt;
            }
            count = next;
        }
        ret as i32
    }
}
*/
