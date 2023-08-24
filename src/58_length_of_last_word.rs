impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut occ = 0;
        let mut last = 0;
        for c in s.chars() {
            if c == ' ' {
                last = if occ > 0 { occ } else { last };
                occ = 0;
            } else {
                occ += 1;
            }
        }
        if occ > 0 { occ } else { last }
    }
}
