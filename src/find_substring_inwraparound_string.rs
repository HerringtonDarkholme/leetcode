pub struct Solution;

/*
Rather than counting how many distinct substrings by hashset
we can think substring by counting its initial.
Given target "abc", "a" will be contribute 3 substring abc, ab, a.
similarly b will contribute 2, and c as 1.
if later we met a consecutive char sequence, like abczab
a will still contribute 3, the a in abc will count 3 and a in ab will count 2.
we don't need to update a's count since all shorter substring with leading a has already be counted
*/
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        if p.is_empty() {
            return 0
        }
        // counting how many unique substrings starting with certain char
        // alphabets[0] stands for how many substrings start with "a"
        const N: usize = 26;
        let mut alphabets = vec![0; N];
        let chars: Vec<_> = p.as_bytes().iter().map(|c| c - 'a' as u8).collect();

        let mut c = 0;
        let is_next_num_consecutive = |i| {
            i + 1 < chars.len() && ((chars[i] + 1) % N as u8) == chars[i + 1]
        };
        for i in 0..chars.len() {
            c += 1;
            if is_next_num_consecutive(i) {
                continue;
            }
            for j in 0..c.min(N) {
                let ch = chars[i + 1 - c + j] as usize;
                alphabets[ch] = alphabets[ch].max((c - j) as i32);
            }
            c = 0;
        }
        alphabets.iter().sum()
    }
}
