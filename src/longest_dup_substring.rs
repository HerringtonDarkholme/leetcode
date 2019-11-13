// lc 1044
use std::collections::HashMap;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let ss: Vec<_> = s.as_bytes().iter().map(|&c| (c - 'a' as u8) as i64).collect();
        let (start, size) = bsearch(&ss);
        s[start..start+size].into()

    }
}

fn bsearch(s: &[i64]) -> (usize, usize) {
    let mut low = 0;
    let mut high = s.len();
    let mut last = 0;
    while low < high {
        let mid = low + (high - low) / 2;
        if let Some(i) = has_dupe_substring(s, mid) {
            last = i;
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if let Some(i) = has_dupe_substring(s, low) {
        (i, low)
    } else if low > 0 {
        (has_dupe_substring(s, low - 1).unwrap(), low - 1)
    } else {
        (0, 0)
    }
}

const BASE: i64 = 26;
const MOD: i64 = 1_000_000_007;

fn has_dupe_substring(s: &[i64], k: usize) -> Option<usize> {
    let mut i = 0;
    let mut hash = 0;
    let mut map = HashMap::new();
    let mut rm = 1;
    while i < k {
        hash = (hash * BASE % MOD + s[i]) % MOD;
        rm = (rm * BASE) % MOD;
        i += 1;
    }
    map.insert(hash, vec![0]);
    while i < s.len() {
        hash = (hash * BASE + s[i]) % MOD;
        hash = (hash - (s[i-k] * rm) % MOD + MOD) % MOD;
        if let Some(occ) = map.get_mut(&hash) {
            for &o in occ.iter() {
                if s[o..o+k] == s[i-k+1..=i] {
                    return Some(o)
                }
            }
            occ.push(i-k+1);
        } else {
            map.insert(hash, vec![i - k + 1]);
        }
        i += 1;
    }
    None
}
