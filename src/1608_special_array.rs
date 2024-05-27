impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in 0..nums.len() {
            let n = (nums.len() - i) as i32;
            if nums[i] >= n && (i == 0 || nums[i - 1] < n) {
                return n
            }
        }
        -1
    }
}
