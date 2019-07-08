pub struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::with_capacity(Solution::factorial(nums.len()));
        Solution::gen_permute(nums.len(), &mut nums, &mut ret);
        ret
    }
    pub fn gen_permute(k: usize, a: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if k <= 1 {
            ret.push(a.clone());
            return;
        }
        for i in 0..k {
            Solution::gen_permute(k - 1, a, ret);
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
        }
    }

    #[inline]
    pub fn factorial(mut num: usize) -> usize {
        let mut r = 1;
        while num > 1 {
            r *= num;
            num -= 1;
        }
        r
    }
}
