pub struct Solution;

use std::collections::{BTreeMap, HashSet};

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Node::from_paths(paths);
        let mut keys = HashSet::new();
        let mut dup = HashSet::new();
        root.generate_key(&mut keys, &mut dup);
        root.dedupe(&dup);
        root.get_path()
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    key: String,
    children: BTreeMap<String, Node>,
}

impl Node {
    fn new() -> Self {
        Self {
            key: String::new(),
            children: BTreeMap::new(),
        }
    }
    fn from_paths(mut paths: Vec<Vec<String>>) -> Self {
        paths.sort_by_key(|p| p.len());
        let mut root = Self::new();
        for mut path in paths {
            path.reverse();
            root.add_dir(path);
        }
        root
    }
    fn add_dir(&mut self, mut path: Vec<String>) {
        if path.len() == 1 {
            let p = path.pop().unwrap();
            self.children.insert(p, Self::new());
            return
        }
        let r = path.pop().expect("path should not be empty");
        let child = self.children.get_mut(&r)
            .expect("parent path should exsit in paths");
        child.add_dir(path);
    }

    fn generate_key(&mut self, keys: &mut HashSet<String>, dup: &mut HashSet<String>) {
        let mut s = String::new();
        for (name, node) in self.children.iter_mut() {
            node.generate_key(keys, dup);
            let key = format!("|{}{}", name, node.key);
            s.push_str(&key);
        }
        self.key = s.clone();
        if keys.contains(&s) {
            dup.insert(s);
        } else if !s.is_empty() {
            keys.insert(s);
        }
    }

    fn dedupe(&mut self, dup: &HashSet<String>) {
        let mut r = BTreeMap::new();
        let mut origin = BTreeMap::new();
        std::mem::swap(&mut origin, &mut self.children);
        for (name,mut  node) in origin.into_iter() {
            if !dup.contains(&node.key) {
                node.dedupe(dup);
                r.insert(name, node);
            }
        }
        self.children = r;
    }

    fn get_path(&self) -> Vec<Vec<String>> {
        let mut r  = vec![];
        for (name, node) in self.children.iter() {
            r.push(vec![name.to_owned()]);
            r.extend(node.get_path().into_iter().map(|mut v| {
                v.insert(0, name.to_owned());
                v
            }));
        }
        r
    }
}
