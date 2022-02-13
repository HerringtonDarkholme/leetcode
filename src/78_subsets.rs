// leetcode 78
pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]];
        for i in 0..nums.len() {
            let mut v = vec![];
            for r in ret.iter() {
                let mut m = r.clone();
                m.push(nums[i]);
                v.push(m);
            }
            ret.append(&mut v);
        }
        ret
    }
}
