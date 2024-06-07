pub struct Solution;

impl Solution {
    pub fn replace_words(mut dict: Vec<String>, sen: String) -> String {
        dict.sort_by(|d1, d2| d1.len().cmp(&d2.len()));
        sen.split(" ").map(|mut w| {
            for d in dict.iter() {
                if w.starts_with(d) {
                    w = d;
                }
            }
            w
        }).collect::<Vec<_>>().join(" ")
    }
}


/*
use std::collections::HashSet;
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut hashs = vec![HashSet::new(); 100];
        for word in dictionary {
            let i = word.len() - 1;
            hashs[i].insert(word);
        }
        let mut ret = vec![];
        for word in sentence.split(' ') {
            let mut added = false;
            for i in 0..100.min(word.len()) {
                let pre = &word[0..=i];
                if hashs[i].contains(pre) {
                    added = true;
                    ret.push(pre);
                    break;
                }
            }
            if !added {
                ret.push(word);
            }
        }
        ret.join(" ")
    }
}

*/
