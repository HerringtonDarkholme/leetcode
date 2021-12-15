struct StreamChecker {
    root: Trie,
    frontiers: Vec<*const Trie>, // self referencing, LOL
}

#[derive(Clone)]
struct Trie {
    is_terminal: bool,
    children: Box<[Option<Trie>]>,
}

impl Default for Trie {
    fn default() -> Self {
        let mut v = vec![None; 26];
        Self {
            is_terminal: false,
            children: v.into_boxed_slice(),
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut s = Self {
            root: Trie::default(),
            frontiers: vec![],
        };
        for word in words {
            build(word.bytes(), &mut s.root);
        }
        s
    }

    fn query(&mut self, letter: char) -> bool {
        let i = ((letter as u8) - b'a') as usize;
        let mut frontiers = vec![];
        unsafe {
            self.frontiers.push(&self.root as *const Trie);

            for &front in self.frontiers.iter() {
                if let Some(t) = (*front).children[i].as_ref() {
                    let t: *const Trie = t;
                    frontiers.push(t);
                }
            }
            self.frontiers = frontiers;
            self.frontiers.iter().any(|&t| (*t).is_terminal)
        }
    }
}

fn build(mut word: impl Iterator<Item=u8>, mut trie: &mut Trie) {
    while let Some(c) = word.next() {
        let i = (c - b'a') as usize;
        trie = trie.children[i].get_or_insert_with(Trie::default);
    }
    trie.is_terminal = true;
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
