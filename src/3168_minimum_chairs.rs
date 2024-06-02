impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut curr = 0;
        let mut min = 0;
        for c in s.chars() {
            if c == 'E' {
                curr += 1;
            } else {
                curr -= 1;
            }
            min = min.max(curr);
        }
        min
    }
}
