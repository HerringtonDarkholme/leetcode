impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ret = String::new();
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        for (&c1, &c2) in w1.iter().zip(w2.iter()) {
            ret.push(c1 as char);
            ret.push(c2 as char);
        }
        if w1.len() > w2.len() {
            ret.extend(w1[w2.len()..].iter().map(|c| *c as char));
        } else {
            ret.extend(w2[w1.len()..].iter().map(|c| *c as char));
        }
        ret
    }
}
