impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut s: Vec<_> = s.chars().collect();
        s.push(' '); // add last char to flush pending
        let mut left = 0;
        let mut ret = vec![];
        for right in 1..s.len() {
            if s[right] == s[left] {
                continue;
            }
            if right - left >= 3 {
                ret.push(vec![left as i32, right as i32 - 1]);
            }
            left = right;
        }
        ret
    }
}
