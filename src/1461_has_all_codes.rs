impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s: Vec<_> = s.chars().collect();
        let mut set = std::collections::HashSet::new();
        for w in s.windows(k as usize) {
            set.insert(w.iter().collect::<String>());
        }
        set.len() == 2_usize.pow(k as u32)
    }
}
