impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut s = s.as_bytes();
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r && s[l] == s[r] {
            while l < r && s[l] == s[l + 1] { l += 1; }
            if l < r { l += 1; }
            while l < r && s[r] == s[r - 1] { r -= 1; }
            if l < r { r -= 1; }
        }
        if l == r && l != 0 && s[l - 1] == s[l] {
            0
        } else {
            (r + 1 - l) as i32
        }
    }
}
