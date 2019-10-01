pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();
        let mut i = 0;
        let mut counts = vec![];
        let mut strs = vec![String::new()];
        while i < chars.len() {
            let c = chars[i];
            match c {
                '0'..='9' => {
                    let mut count = 0;
                    while chars[i] >= '0' && chars[i] <= '9' {
                        count *= 10;
                        count += ((chars[i] as u8) - ('0' as u8)) as usize;
                        i += 1;
                    }
                    counts.push(count);
                },
                '[' => {
                    strs.push(String::new());
                    i += 1;
                },
                ']' => {
                    let s = strs.pop().unwrap();
                    let count = counts.pop().unwrap();
                    let l = strs.len() - 1;
                    strs[l].push_str(&s.repeat(count));
                    i +=1;
                },
                _ => {
                    let l = strs.len() - 1;
                    strs[l].push(c);
                    i += 1;
                }
            }
        }
        strs.pop().unwrap()
    }
}
