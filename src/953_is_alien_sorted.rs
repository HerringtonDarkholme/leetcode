impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order = get_order(order);
        for i in 1..words.len() {
            let w1 = words[i - 1].as_bytes();
            let w2 = words[i].as_bytes();
            if !is_smaller(w1, w2, &order) {
                return false
            }
        }
        true
    }
}

fn get_order(s: String) -> [i8; 26] {
    let mut ret = [0; 26];
    for (i, b) in s.bytes().enumerate() {
        ret[(b - b'a') as usize] = i as i8;
    }
    ret
}

fn is_smaller(w1: &[u8], w2: &[u8], dict: &[i8; 26]) -> bool {
    let len = w1.len().min(w2.len());
    for i in 0..len {
        let c1 = dict[(w1[i] - b'a') as usize];
        let c2 = dict[(w2[i] - b'a') as usize];
        if c1 < c2 {
            return true;
        } else if c1 > c2 {
            return false;
        }
    }
    w1.len() <= w2.len()
}
