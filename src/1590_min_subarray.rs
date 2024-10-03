use std::collections::HashMap;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut d = 0;
        for &n in &nums {
            d += n;
            d %= p;
        }
        if d == 0 { return 0; }
        let mut mods = HashMap::new();
        mods.insert(0, 0);
        let mut s = 0;
        let mut min = nums.len();
        for (i, &n) in nums.iter().enumerate() {
            s += n;
            s %= p;
            let pair = if s >= d { s - d } else { s+p - d };
            if let Some(j) = mods.get(&pair) {
                min = min.min(i + 1 - j);
            }            
            mods.insert(s, i + 1);
        }
        if min == nums.len() { -1 } else { min as i32 }
    }
}

// 4
