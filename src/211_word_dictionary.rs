struct WordDictionary {
    root: Trie,
}

struct Trie {
    ch: char,
    children: Vec<Trie>,
    end: bool,
}
impl Trie {
    fn new(ch: char) -> Self {
        Self {
            ch,
            children: vec![],
            end: false,
        }
    }
    fn has(&self, c: char) -> Option<&Self> {
        self.children.iter().find(|child| child.ch == c)
    }
    fn char(&mut self, c: char) -> &mut Self {
        if self.has(c).is_none() {
            self.children.push(Trie::new(c));
        }
        self.children.iter_mut().find(|child| child.ch == c).unwrap()
    }
    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.char(c);
        }
        node.end = true;
    }
    fn contains(&self, word: String) -> bool {
        let mut frontiner = vec![self];
        for c in word.chars() {
            frontiner = if c == '.' {
                frontiner.into_iter().flat_map(|n| {
                    n.children.iter()
                }).collect()
            } else {
                frontiner.into_iter().filter_map(|n| {
                    n.has(c)
                }).collect()
            };
        }
        frontiner.into_iter().any(|n| n.end)
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self {
            root: Trie::new('*'),
        }
    }

    fn add_word(&mut self, word: String) {
        self.root.insert(word)
    }

    fn search(&self, word: String) -> bool {
        self.root.contains(word)
    }
}
