impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();
        let mut left = 0;
        let mut max = 1;
        for right in 1..nums.len() {
            if nums[right] == nums[right - 1] {
                max = max.max(right - left + 1); // no need to spend k
                continue;
            }
            let diff = nums[right] - nums[right - 1];
            while right > left && k < (right - left) as i32 * diff {
                k += nums[right - 1] - nums[left]; // give back some credit
                left += 1;
            }
            k -= diff * (right - left) as i32; // spend credits
            max = max.max(right - left + 1);
        }   
        max as i32
    }
}

/*

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut sum = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            // count the diff between sum and expected
            if (right - left + 1) as i32 * num - sum > k {
                sum -= unsafe { nums.get_unchecked(left) };
                left += 1; // shrink by one cuz only the longest length is desired
            }
        }   
        (nums.len() - left) as i32
    }
}
*/
