// leetcode 1248
pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = vec![0; nums.len() + 1];
        map[0] = 1;
        let mut sum = 0;
        let mut ret = 0;
        for n in nums {
            sum += n & 1;
            if sum >= k {
                ret += map[(sum - k) as usize];
            }
            map[sum as usize] += 1;
        }
        ret
    }
}
