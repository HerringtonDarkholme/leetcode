use std::collections::HashMap;
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = Trie::new();
        for word in &words {
            root.build(word);
        }
        let mut ret = vec![];
        for word in words {
            ret.push(root.query(word));
        }
        ret
    }
}
#[derive(Clone, Debug)]
struct Trie {
    count: i32,
    kids: HashMap<u8, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            count: 0,
            kids: HashMap::new(),
        }
    }
    fn add(&mut self, c: u8) -> &mut Self {
        if self.kids.get(&c).is_none() {
            self.kids.insert(c, Trie::new());
        }
        let n = self.kids.get_mut(&c).unwrap();
        n.count += 1;
        n
    }
    fn build(&mut self, word: &str) {
        let mut root = self;
        for c in word.bytes() {
            root = root.add(c);
        }
    }
    fn query(&self, word: String) -> i32 {
        let mut root = self;
        let mut ret = 0;
        for c in word.bytes() {
            root = &root.kids[&c];
            ret += root.count;
        }
        ret
    }
}
