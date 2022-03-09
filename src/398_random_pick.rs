use std::collections::HashMap;
use rand::{thread_rng, Rng, rngs::ThreadRng};

struct Solution {
    map: HashMap<i32, Vec<i32>>,
    rng: ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            map.entry(n).or_insert_with(|| vec![]).push(i as i32);
        }
        let mut rng = thread_rng();
        Self {
            map,
            rng,
        }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let v = &self.map[&target];
        let i: usize = self.rng.gen_range(0, v.len());
        v[i]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
