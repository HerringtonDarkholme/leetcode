impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut nums = (1..=n).collect();
        count(&mut nums, n as usize)
    }
}

fn count(nums: &mut Vec<i32>, p: usize) -> i32 {
    let mut r = if p == 0 { 1} else {0};
    for i in 0..p {
        let n = nums[i] as usize;
        if n % p == 0 || p % n == 0 {
            nums.swap(i, p-1);
            r += count(nums, p-1);
            nums.swap(i, p-1);
        }
    }
    r
}
