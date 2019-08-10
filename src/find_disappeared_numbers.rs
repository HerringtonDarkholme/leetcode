pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let mut j = nums[i] as usize;
            nums[i] = 0;
            while j > 0 && nums[j - 1] != j as i32 {
                let temp = nums[j - 1] as usize;
                nums[j - 1] = j as i32;
                j = temp;
            }
        }
        nums.into_iter().enumerate()
            .filter_map(|(i, n)| if i + 1 == n as usize { None } else { Some(i as i32 +1) })
            .collect()
    }
}

#[test]
fn test() {
    for t in vec![
        (vec![4,3,2,7,8,2,3,1], vec![5, 6])
    ] {
        assert_eq!(Solution::find_disappeared_numbers(t.0), t.1);
    }

}
