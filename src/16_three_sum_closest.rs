impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut close = 20000;
        for i in 1..nums.len() - 1 {
            let mut l = 0;
            let mut r = nums.len() - 1;
            while l < i && r > i {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (close - target).abs() {
                    close = sum;
                }
                if sum < target {
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    return sum;
                }
            }
        }
        close
    }
}
