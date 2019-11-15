// leetcode 1248
pub struct Solution;

// note, because number of odd number only increment by 1
// so we can use sliding window
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, mut k: i32) -> i32 {
        at_most(&nums, k) - at_most(&nums, k - 1)
    }
}

fn at_most(nums: &[i32], mut k: i32) -> i32 {
    let mut ret = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        k -= nums[j] % 2;
        while k < 0 {
            k += nums[i] % 2;
            i += 1;
        }
        ret += j - i + 1;
    }
    ret as i32
}

/*
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = vec![0; nums.len() + 1];
        map[0] = 1;
        let mut sum = 0;
        let mut ret = 0;
        for n in nums {
            sum += n & 1;
            if sum >= k {
                ret += map[(sum - k) as usize];
            }
            map[sum as usize] += 1;
        }
        ret
    }
}
*/
