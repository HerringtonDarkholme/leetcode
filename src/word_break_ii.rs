use std::collections::HashSet;

struct Item {
    pos: usize,
    prefix: Vec<String>,
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let dict: HashSet<_> = word_dict.into_iter().collect();
    let max = dict.iter().map(|s| s.len()).fold(0, usize::max);
    let mut dp = vec![false; s.len()+1];
    dp[s.len()] = true;
    for i in 0..s.len() {
        let index = s.len() - i - 1;
        for j in 1..=(i+1).min(max) {
            let word = &s[index..index+j];
            if dict.contains(word) && dp[index+j] {
                dp[index] = true;
                break;
            }
        }
    }
    dp[0]
}


impl Solution {

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        if !word_break(s.clone(), word_dict.clone()) {
            return vec![]
        }
        let dict: HashSet<_> = word_dict.into_iter().collect();
        let mut stack = vec![Item{pos: 0, prefix: vec![]}];
        let mut visited = vec![false; s.len()];
        let mut ret = vec![];
        while !stack.is_empty() {
            let Item{pos, prefix} = stack.pop().unwrap();
            if pos == s.len() {
                ret.push(prefix.join(" "));
                continue;
            }
            for w in dict.iter() {
                if s[pos..].starts_with(w) {
                    let mut p = prefix.clone();
                    p.push(w.to_string());
                    stack.push(Item {
                        pos: pos + w.len(),
                        prefix: p,
                    });
                }
            }
        }
        ret
    }
}

