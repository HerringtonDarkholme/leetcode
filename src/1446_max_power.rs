impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut last = 0;
        let mut cnt = 0;
        let mut max = 1;
        for c in s.bytes() {
            if c == last {
                cnt += 1;
                max = max.max(cnt);
            } else {
                cnt = 1;
                last = c;
            }
        }
        max
    }
}
