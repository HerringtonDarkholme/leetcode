impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 0
        }
        let mut sum = 0;
        let mut n = 0;
        for i in 2..nums.len() {
            if nums[i - 1] - nums[i - 2] == nums[i] - nums[i - 1] {
                n += 1;
            } else {
                n = 0;
            }
            sum += n;
        }
        sum
    }
}
// 1. nums <= 2 , 0
// diff = i-2 - i-1 == i - i-1, n +=1, sum += n
// else n = 0
