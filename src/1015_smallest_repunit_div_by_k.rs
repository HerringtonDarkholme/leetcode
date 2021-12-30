impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // consider remainder: if 11111 is divisible by k
        // it means 11111 % k = 0, we can reuse last computation
        // 1111 % k == (111 * 10 + 1) % k
        // note 10 = 2 * 5 so 11111 will never be divisible
        if k % 2 == 0 || k % 5 == 0 {
            return -1
        }
        let mut cnt = 1;
        let mut r = 1 % k;
        while r != 0 {
            r = (r * 10 + 1) % k;
            cnt += 1;
        }
        cnt
    }
}
