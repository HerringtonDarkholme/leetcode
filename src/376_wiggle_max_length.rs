pub strucrt Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }
        let mut inc = 1;
        let mut h_inc = nums[0];
        let mut des = 1;
        let mut h_des = nums[0];
        for i in 1..nums.len() {
            let n = nums[i];
            let mut n_des = des;
            let mut n_inc = inc;
            let mut nh_inc = h_inc;
            let mut nh_des = h_des;
            if n < h_inc && inc + 1 >= des {
                n_des = inc + 1;
                nh_des = n;
            }
            if n > h_des && des + 1 >= inc {
                n_inc = des + 1;
                nh_inc = n;
            }
            des = n_des;
            inc = n_inc;
            h_inc = nh_inc;
            h_des = nh_des;
        }
        inc.max(des)
    }
}

/*
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut up = 1;
        let mut down = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i -1] {
                up = down + 1;
            } else if nums[i] < nums[i - 1] {
                down = up + 1;
            }
        }
        up.max(down)
    }
}
// inc 1 17 5
// dec 17 5
*/
