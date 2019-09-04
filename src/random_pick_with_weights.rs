use rand::rngs::ThreadRng;
use rand::Rng;
struct Solution {
    dist: Vec<i32>,
    max: i32,
    rng: ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let mut dist = vec![];
        let mut s = 0;
        for n in w {
            s += n;
            dist.push(s);
        }
        Solution {
            dist, max: s,
            rng: rand::thread_rng()
        }
    }

    fn pick_index(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.max);
        for (k, &j) in self.dist.iter().enumerate() {
            if j > i {
                return k as i32
            }
        }
        0
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
