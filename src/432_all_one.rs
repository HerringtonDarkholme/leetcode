use std::collections::{HashMap, HashSet};

#[derive(Clone, Default)]
struct DoubleNode {
    prev: usize,
    next: usize,
    data: HashSet<String>,
}

#[derive(Default)]
struct AllOne {
    map: HashMap<String, usize>,
    link: Vec<DoubleNode>,
    max: usize,
    min: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        Self {
            map: Default::default(),
            link: vec![Default::default()],
            max: 0,
            min: 0,
        }
    }
    
    fn inc(&mut self, key: String) {
        let entry = self.map.entry(key.clone()).or_insert(0);
        *entry += 1;
        if *entry >= self.link.len() {
            self.link.push(DoubleNode {
                prev: 0,
                next: 0,
                data: HashSet::new()
            });
        }
        if *entry == 1 {
            self.link[1].data.insert(key);
            if self.min != 1 {
                self.link[1].next = self.min;
                self.link[self.min].prev = 1;
                self.min = 1;
            }
        } else {
            let n = &mut self.link[*entry - 1];
            n.data.remove(&key);
            let prev = if n.data.is_empty() {
                if self.min == *entry - 1 {
                    self.min = *entry;
                }
                n.prev
            } else {
                *entry - 1
            };
            let next = n.next;
            self.link[prev].next = *entry;
            let nn = &mut self.link[*entry];
            nn.prev = prev;
            nn.data.insert(key);
            if next != *entry {
                nn.next = next;
                self.link[next].prev = *entry;
            }

        }
        self.max = self.max.max(*entry);
    }
    
    fn dec(&mut self, key: String) {
        let entry = self.map.get_mut(&key).unwrap();
        let n = &mut self.link[*entry];
        n.data.remove(&key);
        *entry -= 1;
        let next = if n.data.is_empty() {
            if self.max == *entry + 1 {
                self.max = *entry;
            }
            if self.min == *entry + 1 {
                self.min = if *entry == 0 { n.next } else { *entry };
            }
            n.next
        } else {
            *entry + 1
        };
        let prev = n.prev;
        self.link[next].prev = *entry;
        let nn = &mut self.link[*entry];
        nn.next = next;
        if *entry != 0 {
            nn.data.insert(key);
        }
        if prev != *entry {
            nn.prev = prev;
            self.link[prev].next = *entry;
        }

    }
    
    fn get_max_key(&self) -> String {
        if self.max == 0 {
            String::new()
        } else {
            self.link[self.max].data.iter().next().unwrap().into()
        }
    }
    
    fn get_min_key(&self) -> String {
        if self.min == 0 {
            String::new()
        } else {
            self.link[self.min].data.iter().next().unwrap().into()
        }
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
