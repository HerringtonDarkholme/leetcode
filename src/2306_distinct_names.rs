use std::collections::HashSet;
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut sets = vec![HashSet::new(); 26];
        for idea in &ideas {
            let c = (idea.bytes().next().unwrap() - b'a') as usize;
            sets[c].insert(&idea[1..]);
        }
        let mut ret = 0i64;
        let mut matrix = vec![vec![0; 26]; 26];
        for idea in &ideas {
            let c = (idea.bytes().next().unwrap() - b'a') as usize;
            let suffix = &idea[1..];
            for i in 0..26 {
                if i == c || sets[i].contains(&suffix) {
                    continue;
                }
                ret += 2 * matrix[c][i];
            }
            for idx in 0..26 {
                if c == idx || sets[idx].contains(&suffix) {
                    continue;
                }
                matrix[idx][c] += 1;
            }
        }
        ret
    }
}
