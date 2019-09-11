pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut two = nums.len() - 1;
        let mut front = 0;
        let mut one = 0;
        while front <= two {
            if nums[front] == 2 {
                nums.swap(front, two);
                if front == two {
                    break;
                }
                two -= 1;

            } else if nums[front] == 0 {
                nums.swap(front, one);
                front += 1;
                one += 1;
            } else {
                front += 1;
            }
        }
    }
}
