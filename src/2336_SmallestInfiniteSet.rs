use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    back: BTreeSet<i32>,
    cnt: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self {
            back: BTreeSet::new(),
            cnt: 0,
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if self.back.is_empty() {
            self.cnt += 1;
            self.cnt
        } else {
            let ret = *self.back.iter().next().unwrap();
            self.back.remove(&ret);
            ret
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if num == self.cnt {
            self.cnt -= 1;
        } else if num < self.cnt {
            self.back.insert(num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
