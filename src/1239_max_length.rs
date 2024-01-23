use std::collections::HashSet;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        set.insert(0);
        let mut ret = 0;
        for s in arr {
            let i = match encode(s) {
                Some(i) => i,
                None => continue,
            };
            let mut next = set.clone();
            for j in set {
                if i & j != 0 {
                    continue;
                }
                let new: i32 = i | j;
                next.insert(new);
                ret = ret.max(new.count_ones());
            }
            set = next;
        }
        ret as i32
    }
}
fn encode(s: String) -> Option<i32> {
    let mut ret = 0;
    let mut has_dup = false;
    for b in s.bytes() {
        let i = (b - b'a') as usize;
        if ret & (1 << i) != 0 {
            return None
        }
        ret |= 1 << i;
    }
    Some(ret)
}
