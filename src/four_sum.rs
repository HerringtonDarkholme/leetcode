struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        nums.sort();
        Solution::search(&mut ret, &nums, &mut vec![], target);
        ret
    }

    fn search(ret: &mut Vec<Vec<i32>>, nums: &[i32], cand: &mut Vec<i32>, target: i32) {
        let sum = cand.iter().sum::<i32>();
        if cand.len() == 4 {
            if sum == target {
                ret.push(cand.clone());
            }
            return;
        }
        if nums.is_empty() {
            return;
        }
        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if sum + nums[i] > target && nums[i] > 0 {
                return;
            }
            cand.push(nums[i]);
            Solution::search(ret, &nums[(i+1)..], cand, target);
            cand.pop();
        }
    }
}
