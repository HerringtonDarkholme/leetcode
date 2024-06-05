use std::collections::HashMap;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut words = words.into_iter();
        let mut counter = word_to_counter(words.next().unwrap());
        for word in words {
            let c = word_to_counter(word);
            let mut next = HashMap::new();
            for (k, v) in c {
                let Some(&prev) = counter.get(&k) else { continue };
                next.insert(k, prev.min(v));
            }
            counter = next;
        }
        let mut ret = vec![];
        for (k, v) in counter {
            for _ in 0..v {
                ret.push(k.to_string());
            }
        }
        ret
    }
}
fn word_to_counter(word: String) -> HashMap<char, i32> {
    let mut r = HashMap::new();
    for c in word.chars() {
        *r.entry(c).or_insert(0) += 1;
    }
    r
}
