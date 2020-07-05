struct Solution;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false
        }
        contains_nearby_almost_duplicate(nums, k as usize, t as isize)
    }
}

fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: usize, t: isize) -> bool {
    use std::collections::HashMap;
    let mut buckets = HashMap::new();
    for i in 0..nums.len() {
        let n = nums[i] as isize;
        let b = n / (t+1);
        if
            buckets.get(&(b)).filter(|&m| (m-n).abs() <= t).is_some() ||
            buckets.get(&(b-1)).filter(|&m| (m-n).abs() <= t).is_some() ||
            buckets.get(&(b+1)).filter(|&m| (m-n).abs() <= t).is_some()
        {
            return true
        }
        buckets.insert(b, n);
        if buckets.len() > k {
            let last = nums[i - k] as isize / (t+1);
            buckets.remove(&last);
        }
    }
    false
}

#[test]
fn test() {
    assert_eq!(contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0), true);
}
