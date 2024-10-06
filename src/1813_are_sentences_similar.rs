impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let (sentence1, sentence2) = if sentence1.len() > sentence2.len() {
            (sentence1, sentence2)
        } else {
            (sentence2, sentence1)
        };
        let mut words1: Vec<_> = sentence1.split(' ').collect();
        let mut words2: Vec<_> = sentence2.split(' ').collect();
        let mut j = 0;
        for i in 0..words1.len() {
            if j < words2.len() && words1[i] == words2[j] {
                j += 1;
            } else {
                break;
            }
        }
        for k in (j..words2.len()).rev() {
            let i = words1.len() - (words2.len() - k);
            if words2[k] != words1[i] {
                return false;
            }
        }
        true
    }
}
