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

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let e = find_last_decreasing(nums);
        if e > 0 {
            swap_next_large(nums, e);
        }
        reverse_trailing(nums, e);
    }
}

fn find_last_decreasing(nums: &[i32]) -> usize {
    for i in (1..nums.len()).rev() {
        if nums[i - 1] < nums[i] {
            return i
        }
    }
    0
}
fn swap_next_large(nums: &mut [i32], e: usize) {
    for i in (e..nums.len()).rev() {
        if nums[i] > nums[e - 1] {
            nums.swap(i, e - 1);
            break;
        }
    }
}
fn reverse_trailing(nums: &mut [i32], e: usize) {
    let mut left = e;
    let mut right = nums.len() - 1;
    while left < right {
        nums.swap(left, right);
        left += 1;
        right -= 1;
    }
}
// 1 2 3 9
// 1 2 4 3
// 1 3 2 4
// 1 3 4 2
// 1 2 3 4 5 6 3 2
//           e
