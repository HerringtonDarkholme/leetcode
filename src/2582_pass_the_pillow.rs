impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let t = time % (2 * n - 2);
        if t > n - 1 {
            2 * n - t - 1
        } else {
            t + 1
        }
    }
}
