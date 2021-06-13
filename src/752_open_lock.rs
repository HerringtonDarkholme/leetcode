use std::collections::HashSet;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut seen: HashSet<_> = deadends.into_iter().collect();
        if seen.contains("0000") {
            return -1
        }
        seen.insert("0000".to_string());
        let mut frontier = vec!["0000".to_string()];
        let mut steps = 0;
        while !frontier.is_empty() {
            let mut next = vec![];
            for f in frontier {
                if f == target {
                    return steps
                }
                for i in 0..4 {
                    for &j in &[-1, 1] {
                        let mut n = f.clone();
                        let mut c = unsafe { n.as_mut_vec() };
                        c[i] = if c[i] as i8 + j > '9' as i8 {
                            '0' as u8
                        } else if c[i] as i8 + j < '0' as i8 {
                            '9' as u8
                        } else {
                            (c[i] as i8 + j) as u8
                        };
                        if seen.contains(&n) {
                            continue;
                        }
                        seen.insert(n.clone());
                        next.push(n);
                    }
                }
            }
            frontier = next;
            steps += 1;
        }
        -1
    }
}
