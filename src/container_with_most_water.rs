pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut max = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while l < r {
            let left = height[l];
            let right = height[r];
            max = std::cmp::max(std::cmp::min(left, right) * (r-l) as i32, max);
            if left < right {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max
    }
}

#[test]
fn test() {
    for (case, exp) in vec![
        (vec![1,8,6,2,5,4,8,3,7], 49),
        (vec![1,2,3,4,5,6,7,8,9], 20),
        (vec![1,1,1,1,1,1,1,1,1], 8),
        (vec![1,4,6,8,4,3,7,3,2], 24),
    ] {
        assert_eq!(Solution::max_area(case), exp);
    }
}
