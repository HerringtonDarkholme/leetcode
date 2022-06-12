impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut mismatch1 = -1;
        let mut mismatch2 = -1;
        if s1.len() != s2.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        for i in 0..s1.len() {
            if s1[i] == s2[i] {
                continue;
            }
            if mismatch1 < 0 {
                mismatch1 = i as i32;
            } else if mismatch2 < 0 {
                mismatch2 = i as i32;
            } else {
                return false;
            }
        }
        if mismatch1 < 0 && mismatch2 < 0 {
            true
        } else if mismatch2 >= 0 && mismatch2 >= 0 {
            s1[mismatch1 as usize] == s2[mismatch2 as usize] && s1[mismatch2 as usize] == s2[mismatch1 as usize]
        } else {
            false
        }
    }
}
