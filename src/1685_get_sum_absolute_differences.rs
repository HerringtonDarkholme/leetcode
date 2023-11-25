impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let sum: i32 = nums.iter().sum();
        let mut ret = vec![];
        let mut prefix = 0;
        let len = nums.len() as i32;
        for (i, n) in nums.into_iter().enumerate() {
            let i = i as i32;
            ret.push(
                n * i - prefix + sum - prefix - n * (len - i)
            );
            prefix += n;
        }
        ret
    }
}
