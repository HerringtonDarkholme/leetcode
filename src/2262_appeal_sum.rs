struct NumArray {
    arr: Vec<i64>,
    nums: Vec<i64>,
}

#[inline]
fn lsb(i: i64) -> i64 {
    i & (-i)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i64>) -> Self {
        let n = nums.len();
        let mut s = NumArray {
            arr: vec![0; n + 1],
            nums: vec![0; n],
        };
        for i in 0..nums.len() {
            s.update(i as i64, nums[i]);
        }
        s
    }

    fn update(&mut self, mut i: i64, val: i64) {
        let v = val - self.nums[i as usize];
        self.nums[i as usize] = val;
        let mut i = (i + 1) as usize;
        while i < self.arr.len() {
            self.arr[i] += v;
            i += lsb(i as i64) as usize;
        }
    }
    fn sum(&self, i: i64) -> i64 {
        let mut i = (i + 1) as usize;
        let mut sum = 0;
        while i > 0 {
            sum += self.arr[i];
            i -= lsb(i as i64) as usize;
        }
        sum
    }

    fn sum_range(&self, i: i64, j: i64) -> i64 {
        self.sum(j) - self.sum(i - 1)
    }
}

// a b c b c a
use std::collections::HashMap;
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let s: Vec<_> = s.chars().collect();
        let mut occ = HashMap::new();
        let mut ret = 0;
        let mut num_arr = NumArray::new(vec![0; s.len()]);
        for i in 0..s.len() {
            let c = s[i];
            let s = occ.get(&c).map(|j| j + 1).unwrap_or(0);
            for k in s..=i {
                num_arr.update(k as i64, num_arr.nums[k] + 1);
            }
            occ.insert(c, i);
            ret += num_arr.sum(i as i64);
        }
        ret as i64
    }
}
