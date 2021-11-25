impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut max = i32::MIN;
        for n in nums {
            s += n;
            max = max.max(s);
            s = 0.max(s);
        }
        max
    }
}
