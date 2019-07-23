pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() {
            return;
        }
        let l = nums.len();
        let k = k as usize % l;
        if k == 0 {
            return;
        }
        let gcd = {
            let mut a = k;
            let mut b = l;
            while a != 0 {
                let temp = a;
                a = b % a;
                b = temp;
            }
            b
        };
        for s in 0..gcd {
            let mut i = s;
            let mut temp = nums[i];
            loop {
                let j = (i + k) % l;
                let t = temp;
                temp = nums[j];
                nums[j] = t;
                i = j;
                if i == s {
                    break;
                }
            }
        }
    }
}
