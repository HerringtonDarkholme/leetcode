impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false
        }
        let mut targets = vec![sum / k; k as usize];
        can_divide(&nums, &mut targets)
    }
}

fn can_divide(nums: &[i32], targets: &mut Vec<i32>) -> bool {
    if nums.len() == 0 {
        return targets.iter().all(|&x| x == 0)
    }
    let n = nums[0];
    for i in 0..targets.len() {
        if targets[i] < n {
            continue;
        }
        targets[i] -= n;
        if can_divide(&nums[1..], targets) {
            return true
        }
        targets[i] += n;
    }
    false
}
