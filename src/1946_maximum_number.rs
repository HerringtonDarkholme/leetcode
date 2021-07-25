struct Solution;

impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut chars: Vec<_> = num.chars().map(char_to_num).collect();
        let mut swapped = false;
        for i in 0..chars.len() {
            let c = chars[i];
            let changed = change[c] as usize;
            if changed <= c {
                if swapped && changed < c {
                    break;
                }
                continue;
            }
            swapped = true;
            chars[i] = changed;
        }
        chars.into_iter().map(num_to_char).collect()
    }
}

fn char_to_num(c: char) -> usize {
    c as usize - '0' as usize
}

fn num_to_char(c: usize) -> char {
    (c as u8 + '0' as u8) as char
}
