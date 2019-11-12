// Leetcode 806
pub struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut line = 0;
        let mut curr = 0;
        for &b in bytes {
            let width = widths[char_to_idx(b)];
            if width + curr > 100 {
                line += 1;
                curr = width;
            } else {
                curr += width;
            }
        }
        vec![line + 1, curr]
    }
}

fn char_to_idx(c: u8) -> usize {
    (c - 'a' as u8) as usize
}
