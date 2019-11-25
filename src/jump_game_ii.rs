pub struct Solution;

// greedy, among jump candidates
// jump to the one can takes me to the furthest
impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            nums[i] = (nums[i] + i as i32).min(nums.len() as i32 - 1);
        }
        let mut i = 0;
        let mut m = 0;
        loop {
            if i == nums.len() - 1 {
                break m
            }
            let mut next_i = i + 1;
            // this solution is hard to compute time complexity
            for j in i+1..nums.len().min(nums[i] as usize + 1) {
                if nums[next_i] <= nums[j] {
                    next_i = j;
                }
            }
            i = next_i;
            m += 1;
        }
    }
}
