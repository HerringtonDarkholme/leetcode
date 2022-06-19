
impl Trie {
    fn new() -> Self {
        Self {
            children: vec![None; 27],
        }
    }
    fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            return;
        }
        let i = (word[0] - b'a') as usize;
        if let Some(n) = &mut self.children[i] {
            n.insert(&word[1..]);
        } else {
            let mut inner = Trie::new();
            inner.insert(&word[1..]);
            self.children[i] = Some(inner);
        }
    }
    fn exist(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            return self.pos;
        }
        let i = (word[0] - b'a') as usize;
        if let Some(n) = &self.children[i] {
            n.exist(&word[1..])
        } else {
            -1
        }
    }
}
