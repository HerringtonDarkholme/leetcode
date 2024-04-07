impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left = vec![];
        let mut star = vec![];
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                left.push(i);
            } else if c == '*' {
                star.push(i)
            } else if !left.is_empty() {
                left.pop();
            } else if !star.is_empty() {
                star.pop();
            } else {
                return false;
            }
        }
        let mut j = 0;
        for l in left {
            while j < star.len() && star[j] <= l {
                j += 1;
            }
            if j == star.len() {
                return false;
            }
            j += 1;
        }
        true
    }
}

// (**)
