impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut c = 0;
        let mut i = 0;
        for greed in g {
            while i < s.len() && greed > s[i] {
                i += 1;
            }
            if i < s.len() {
                c += 1;
                i += 1;
            }
        }
        c
    }
}
