impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut ret = vec![0; nums.len()];
        if nums.len() < 2 {
            return 1;
        }
        ret[0] = 1;
        ret[1] = if nums[1] == nums[0] { 2 } else { 0 };
        for i in 2..nums.len() {
            if nums[i] == nums[i - 1] {
                ret[i] = (ret[i - 1] + 1).min(ret[i - 2] + 2);
            } else {
                ret[i] = ret[i - 2];
            }
        }
        ret[nums.len() - 1]
    }
}


// min_del(i) = if nums[i] == nums[i+1]:
//   (min_del(i+2) + 2 , min_del(i+1) + 1)
// else min_del(i+2)
