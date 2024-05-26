impl Solution {
  pub fn compressed_string(word: String) -> String {
    let mut ret = String::new();
    let word = word.as_bytes();
    let mut i = 0;
    while i < word.len() {
      let curr = word[i];
      let start = i;
      while i + 1 < word.len() && i - start + 1 < 9 && word[i + 1] == curr {
        i += 1;
      }
      i += 1;
      ret.push((b'0' + (i - start) as u8) as char);
    }
    ret
  }
}
