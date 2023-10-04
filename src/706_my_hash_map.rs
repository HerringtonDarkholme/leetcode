struct MyHashMap {
    // key, value pair
    inner: Vec<(i32, i32)>,
}
impl MyHashMap {
    fn new() -> Self {
        Self {
            inner: vec![(-1, -1); 10001],
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        let mut hash = (key as usize) % self.inner.len();
        while self.inner[hash].0 != -1 && self.inner[hash].0 != key {
            hash = (hash + 1) % self.inner.len();
        }
        self.inner[hash] = (key, value);
    }
    fn get(&self, key: i32) -> i32 {
        let mut hash = (key as usize) % self.inner.len();
        let mut i = 0;
        while self.inner[hash].0 != key && i < self.inner.len() {
            hash = (hash + 1) % self.inner.len();
            i += 1;
        }
        if self.inner[hash].0 == key {
            self.inner[hash].1
        } else {
            -1
        }
    }
    fn remove(&mut self, key: i32) {
        let mut hash = (key as usize) % self.inner.len();
        let mut i = 0;
        while self.inner[hash].0 != key && i < self.inner.len() {
            hash = (hash + 1) % self.inner.len();
            i += 1;
        }
        if self.inner[hash].0 == key {
            self.inner[hash].0 = -1;
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
