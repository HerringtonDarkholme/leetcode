impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let k = k as usize;
        for i in 0..(nums.len() + 1 - k) {
            if nums[i] == 1 { continue; }
            ret += 1;
            for j in i..(i + k) {
                nums[j] = 1 - nums[j];
            }
        }
        for i in (nums.len() - k)..nums.len() {
            if nums[i] == 0 {
                return -1;
            }
        }
        ret
    }
}
