use std::collections::HashMap;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut ret = 0;
        let mut map = HashMap::new();
        let mut sig = 0; // compute a sig for current vowels parity
        map.insert(sig, -1);
        for (i, c) in s.chars().enumerate() {
            sig = match c {
                'a' => sig ^ 1,
                'e' => sig ^ 2,
                'i' => sig ^ 4,
                'o' => sig ^ 8,
                'u' => sig ^ 16,
                _ => sig,
            };
            if let Some(&n) = map.get(&sig) {
                ret = ret.max(i as i32 - n);
            } else {
                map.insert(sig, i as i32);
            }
        }
        ret
    }
}
