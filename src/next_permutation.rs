pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return
        }
        let i = find_last_permutation(nums);
        if i == 0 {
            reverse(nums);
            return
        }
        let target = nums[i - 1];
        let swap = find_last_larger(nums, target);
        nums.swap(i - 1, swap);
        reverse(&mut nums[i..]);
    }
}

fn find_last_permutation(nums: &mut Vec<i32>) -> usize {
    let mut last = nums[nums.len() - 1];
    let l = nums.len();
    let mut i = l - 2;
    loop {
        if nums[i] < last {
            break i + 1
        }
        last = nums[i];
        if i == 0 {
            break i
        }
        i -= 1;
    }
}

fn find_last_larger(nums: &Vec<i32>, target: i32) -> usize {
    let mut i = nums.len() - 1;
    while i >= 0 {
        if nums[i] > target {
            return i
        }
        i -= 1;
    }
    0
}

fn reverse(nums: &mut [i32]) {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
