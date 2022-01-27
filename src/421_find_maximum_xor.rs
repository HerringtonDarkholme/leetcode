impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        find_max_xor(nums)
    }
}

#[derive(Default)]
struct Trie {
    zero: Option<Box<Trie>>,
    one: Option<Box<Trie>>,
    end: Option<i32>
}
impl Trie {
    fn has(&self, bit: i32) -> Option<&Self> {
        if bit == 0 {
            self.zero.as_ref().map(|t| &**t)
        } else {
            self.one.as_ref().map(|t| &**t)
        }
    }
    fn insert(&mut self, bit: i32) -> &mut Self {
        if bit == 0 && self.zero.is_none() {
            self.zero = Some(Box::new(Trie::default()))
        } else if bit == 1 && self.one.is_none() {
            self.one = Some(Box::new(Trie::default()))
        }
        if bit == 0 {
            self.zero.as_mut().unwrap()
        } else {
            self.one.as_mut().unwrap()
        }
    }
    fn conclude(&mut self, num: i32) {
        let mut node = self;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            node = node.insert(bit);
        }
        node.end = Some(num)
    }
}
fn build_trie(nums: &[i32]) -> Trie {
    let mut trie = Trie::default();
    for &num in nums {
        trie.conclude(num);
    }
    trie
}
fn find_max_xor(nums: Vec<i32>) -> i32 {
    let trie = build_trie(&nums);
    let mut ans = 0;
    for num in nums {
        let mut node = &trie;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            if let Some(next) = node.has(1 - bit) {
                node = next;
            } else {
                node = node.has(bit).unwrap();
            }
        }
        ans = ans.max(num ^ node.end.unwrap());
    }
    ans
}
