use std::collections::HashSet;

impl Solution {
    pub fn maximum_removals(s: String, p: String, r: Vec<i32>) -> i32 {
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();
        let mut low = 0;
        let mut high = r.len();
        while low < high {
            let mid = low + (high - low) / 2;
            let r: HashSet<_> = r[..mid].iter().map(|&i| i as usize).collect();
            if is_subseq(&s, &p, r) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let r: HashSet<_> = r[..low].iter().map(|&i| i as usize).collect();
        if is_subseq(&s, &p, r) {
            low as i32
        } else {
            low as i32 - 1
        }
    }
}

fn is_subseq(s: &Vec<char>, p: &Vec<char>, r: HashSet<usize>) -> bool {
    let mut i = 0;
    for j in 0..p.len() {
        let c = p[j];
        while i < s.len() && (s[i] != c || r.contains(&i)) {
            i += 1;
        }
        if i >= s.len() {
            return false
        }
        i += 1;
    }
    true
}
