impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let dict = preprocess(s);
        words
            .into_iter()
            .filter(|w| hit(&dict, w))
            .count() as i32
    }
}

fn preprocess(s: String) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![]; 26];
    let cs = s.as_bytes();
    for i in 0..cs.len() {
        let c = cs[i] - 'a' as u8;
        ret[c as usize].push(i);
    }
    ret
}

fn hit(dict: &Vec<Vec<usize>>, w: &String) -> bool {
    let mut p = 0;
    for &c in w.as_bytes() {
        let i = c as usize - 'a' as usize;
        let d = &dict[i];
        match d.binary_search(&p) {
            Ok(pos) => p = d[pos] + 1,
            Err(pos) => if pos >= d.len() {
                return false
            } else {
                p = d[pos] + 1
            }
        }
    }
    true
}
