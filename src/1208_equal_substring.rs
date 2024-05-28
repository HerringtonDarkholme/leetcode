impl Solution {
    pub fn equal_substring(s: String, t: String, mut cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut ret = 0;
        for j in 0..s.len() {
            let c = (s[j] as i32 - t[j] as i32).abs() as i32;
            while cost < c {
                cost += (s[i] as i32 - t[i] as i32).abs() as i32;
                i += 1;
            }
            ret = ret.max(j - i + 1);
            cost -= c;
        }
        ret as i32
    }
}
