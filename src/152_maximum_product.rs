struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        max_product(nums)
    }
}

fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0]
    }
    let mut pos = 0;
    let mut neg = 0;
    let mut max = nums[0];
    for n in nums {
        if n == 0 {
            pos = 0;
            neg = 0;
        } else if n > 0 {
            pos = if pos == 0 { n } else { n * pos };
            neg = neg * n;
        } else {
            let p = pos;
            pos = neg * n;
            neg = if p == 0 { n } else { n * p };
        }
        max = max.max(pos);
    }
    max
}
