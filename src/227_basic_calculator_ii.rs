
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut ret = 0;
        let mut multi = 1;
        let mut op = None;
        let mut curr = 0;
        let mut sign = 1;
        for c in s.chars() {
            if c == ' ' {
                continue;
            }
            if c == '+' || c == '-' {
                if let Some('*') = op {
                    ret += multi * curr * sign;
                } else if let Some('/') = op {
                    ret += multi / curr * sign;
                } else {
                    ret += curr * sign;
                }
                op = None;
                multi = 1;
                curr = 0;
                sign = if c == '+' { 1 } else { -1 };
            } else if c == '*' || c == '/' {
                if let Some('*') = op {
                    multi *= curr;
                } else if let Some('/') = op {
                    multi /= curr;
                } else {
                    multi = curr;
                }
                op = Some(c);
                curr = 0;
            } else {
                curr = curr * 10 + (c as u8 - b'0') as i32;
            }
        }
        if let Some('*') = op {
            ret + multi * curr * sign
        } else if let Some('/') = op {
            ret + multi / curr * sign
        } else {
            ret + curr * sign
        }
    }
}
