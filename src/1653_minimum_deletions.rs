impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a = s.chars().filter(|c| *c == 'a').count();
        let mut b = 0;
        let mut ret = a;
        for (i, c) in s.chars().enumerate() {
            if c == 'a' {
                a -= 1;
            } else {
                b += 1;
            }
            ret = ret.min(a + b);
        }
        ret as i32
    }
}
