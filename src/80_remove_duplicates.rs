impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut j = 1;
        let mut occ = 1;
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                nums[j] = nums[i];
                j += 1;
                occ = 1;
            } else {
                occ += 1;
                if occ <= 2 {
                    nums[j] = nums[i];
                    j += 1;
                }
            }
        }
        j as i32
    }
}
