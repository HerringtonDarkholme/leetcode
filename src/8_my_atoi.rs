impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut i = 0i32;
        let mut sign = 1;
        let mut seen = false;
        for c in s.bytes().skip_while(|&c| c == b' ') {
            if !seen {
                seen = true;
                if c == b'+' {
                    continue;
                }
                if c == b'-' {
                    sign = -1;
                    continue;
                }  
            }
            seen = true;
            if c >= b'0' && c <= b'9' {
                match i.checked_mul(10).and_then(|d| d.checked_add(((c - b'0') as i32) * sign)) {
                    Some(j) => i = j,
                    None => if sign > 0 {
                        i = i32::MAX;
                        break;
                    } else {
                        i = i32::MIN;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        i
    }
}
