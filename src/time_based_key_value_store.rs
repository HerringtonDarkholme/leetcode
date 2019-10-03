use std::collections::HashMap;
struct TimeMap {
    inner: HashMap<String, Vec<(i32, String)>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.inner.entry(key)
            .or_insert(vec![])
            .push((timestamp, value))
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.inner.get(&key) {
            match v.binary_search_by_key(
                &timestamp,
                |s| { s.0 }
            ) {
                Ok(idx) => v[idx].1.clone(),
                Err(idx) => if idx == 0 {
                    String::new()
                } else {
                    v[idx - 1].1.clone()
                }
            }
        } else {
            String::new()
        }
    }
}
