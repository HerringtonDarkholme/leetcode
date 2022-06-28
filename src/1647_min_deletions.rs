impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        cnt.sort_by_key(|&c| std::cmp::Reverse(c));
        let mut seen = std::collections::HashSet::new();
        let mut ret = 0;
        for mut c in cnt {
            while seen.contains(&c) {
                c -= 1;
                ret += 1;
            }
            if c != 0 {
                seen.insert(c);
            }
        }
        ret
    }
}
