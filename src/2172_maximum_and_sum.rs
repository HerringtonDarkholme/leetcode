use std::collections::HashMap;
impl Solution {
    pub fn maximum_and_sum(mut nums: Vec<i32>, num_slots: i32) -> i32 {
        let mut slots = vec![0; num_slots as usize];
        nums.sort();
        nums.reverse();
        let mut bound = nums.iter().sum();
        let mut dp = HashMap::new();
        max_and_sum(&nums, &mut slots, &mut dp, bound)
    }
}
pub fn max_and_sum(nums: &[i32], slots: &mut Vec<i32>, dp: &mut HashMap<String, i32>, bound: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let key = format!("{}_{:?}", nums.len(), slots);
    if let Some(&r) = dp.get(&key) {
        return r;
    }
    let mut max = 0;
    let num = nums[0];
    for i in 0..slots.len() {
        if slots[i] >= 2 {
            continue;
        }
        let ret = (i as i32 + 1) & num;
        if ret + bound - num <= max {
            continue;
        }
        slots[i] += 1;
        max = max.max(ret + max_and_sum(&nums[1..], slots, dp, bound - num));
        slots[i] -= 1;
    }
    dp.insert(key, max);
    max
}
