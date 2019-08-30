pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut i = 0;
        let mut ret = vec![vec![]];
        while i < nums.len() {
            let mut count = 0;
            while count + i < nums.len() && nums[i] == nums[i+count] {
                count += 1;
            }
            let mut prev_n = ret.len();
            for j in 0..prev_n {
                let mut prev = ret[j].clone();
                for k in 0..count {
                    prev.push(nums[i]);
                    ret.push(prev.clone());
                }
            }
            i += count;
        }
        ret
    }
}
