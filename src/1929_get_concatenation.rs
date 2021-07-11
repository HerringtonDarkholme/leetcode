impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let b = nums.clone();
        nums.extend(b);
        nums
    }
}
