use std::collections::{VecDeque, HashSet, HashMap};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut stack = VecDeque::new();
        let begin: Vec<_> = begin_word.chars().collect();
        let end: Vec<_> = end_word.chars().collect();
        let mut words: Vec<Vec<_>> = word_list.into_iter()
            .map(|v| v.chars().collect())
            .collect();
        words.push(begin.clone());
        let map = make_map(words);        
        let mut visited = HashSet::new();
        stack.push_back((begin, 1));
        while !stack.is_empty() {
            let (next, level) = stack.pop_front().unwrap();
            visited.insert(next.clone());
            if next == end {
                return level;
            }
            for w in map[&next].iter() {
                if !visited.contains(w) {
                    stack.push_back((w.clone(), level+1));
                }
            }
        }
        0
    }
}

fn make_map(words: Vec<Vec<char>>) -> HashMap<Vec<char>, Vec<Vec<char>>> {
    let mut ret = HashMap::new();
    for i in 0..words.len() {
        ret.insert(words[i].clone(), vec![]);
        for j in 0..words.len() {
            if is_adjacent(&words[i], &words[j]) {
                ret.get_mut(&words[i]).unwrap().push(words[j].clone());
            }
        }
    }
    ret
}

fn is_adjacent(w1: &[char], w2: &[char]) -> bool {
    let mut same = true;
    for i in 0..w1.len() {
        if w1[i] != w2[i] {
            if !same {
                return false
            }
            same = false;
        }
    }
    !same
}
