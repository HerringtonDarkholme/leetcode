impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s: Vec<_> = s.chars().collect();
        valid_palindrome(&s, 0, s.len() - 1, true)
    }
}

fn valid_palindrome(s: &[char], mut l: usize, mut r: usize, can_skip: bool) -> bool {
    while l < r {
        if s[l] == s[r] {
            l += 1;
            r -= 1;
            continue;
        }
        if !can_skip {
            return false;
        }
        return valid_palindrome(s, l + 1, r, false) || valid_palindrome(s, l, r - 1, false)
    }
    true
}
