use std::collections::HashMap;
struct Encrypter {
    key_map: Vec<String>,
    dict: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Encrypter {

    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let key_map = make_key_map(&keys, &values);
        let dict = make_dict(dictionary, &key_map);
        Self {
            key_map,
            dict,
        }
    }

    fn encrypt(&self, word1: String) -> String {
        encrypt(&self.key_map, word1)
    }

    fn decrypt(&self, word2: String) -> i32 {
        *self.dict.get(&word2).unwrap_or(&0)
    }
}

fn make_key_map(keys: &[char], values: &[String]) -> Vec<String> {
    let mut ret = vec![String::new(); 26];
    for i in 0..keys.len() {
        let c = (keys[i] as u8 - b'a') as usize;
        ret[c] = values[i].clone();
    }
    ret
}

fn make_dict(originals: Vec<String>, key_map: &[String]) -> HashMap<String, i32> {
    let mut ret = HashMap::new();
    for s in originals {
        let d = encrypt(key_map, s);
        *ret.entry(d).or_insert(0) += 1;
    }
    ret
}

fn encrypt(key_map: &[String], word1: String) -> String {
    let mut ret = String::new();
    for c in word1.bytes() {
        let s = &key_map[(c - b'a') as usize];
        ret.push_str(s);
    }
    ret
}
