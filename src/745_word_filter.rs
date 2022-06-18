#[derive(Clone)]
struct Trie {
    children: Vec<Option<Trie>>,
    pos: i32,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: vec![None; 27],
            pos: -1,
        }
    }
    fn insert(&mut self, word: &[u8], pos: usize) {
        if word.is_empty() {
            return;
        }
        let i = (word[0] - b'a') as usize;
        if let Some(n) = &mut self.children[i] {
            n.pos = pos as i32;
            n.insert(&word[1..], pos);
        } else {
            let mut inner = Trie::new();
            inner.pos = pos as i32;
            inner.insert(&word[1..], pos);
            self.children[i] = Some(inner);
        }
    }
    fn exist(&self, word: &[u8]) -> i32 {
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

// a p p l e
//   x i s
struct WordFilter {
    root: Trie,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::new();
        for (weight, word) in words.into_iter().enumerate() {
            let mut word: Vec<_> = word.bytes().collect();
            word.insert(0, b'{');
            root.insert(&word, weight);
            let len = word.len();
            for i in 0..len {
                word.insert(0, word[word.len() - 1 - i]);
                root.insert(&word, weight);
            }
        }
        Self {
            root,
        }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut r = format!("{}{{{}", suffix, prefix);
        self.root.exist(r.as_bytes())
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */
