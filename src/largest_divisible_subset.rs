pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut rets: Vec<Vec<i32>> = vec![];
        for i in nums {
            let mut r = None;
            for v in rets.iter() {
                if i % v.last().unwrap() == 0 {
                    let mut temp = v.clone();
                    temp.push(i);
                    r = Some(temp);
                    break;
                }
            }
            rets.push(r.unwrap_or(vec![i]));
            rets.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
        }
        rets.into_iter().next().unwrap_or(vec![])
    }
}
