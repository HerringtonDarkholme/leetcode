pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut i = g.len();
        let mut j = s.len();
        let mut r = 0;
        while i != 0 && j != 0 {
            if g[i - 1] <= s[j - 1] {
                r += 1;
                i -= 1;
                j -= 1;
            } else {
                i -= 1;
            }
        }
        r
    }
}
