impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut r = 1;
        let mut min = nums[0];
        for i in 1..nums.len() {
            if nums[i] - min > k {
                min = nums[i];
                r += 1;
            }
        }
        r
    }
}

