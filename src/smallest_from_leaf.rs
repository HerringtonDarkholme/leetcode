// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut set = HashSet::new();
        if root.is_none() {
            return String::new()
        }
        dfs(root.unwrap(), String::new(), &mut set);
        let mut str: Vec<_> = set.into_iter().collect();
        let mut small = str[0].clone();
        for i in 1..str.len() {
            small = small_str(&small, &str[i]).clone();
        }
        small
    }
}
fn dfs(root: Rc<RefCell<TreeNode>>, mut suffix: String,set: &mut HashSet<String>) {
    let mut root = root.borrow_mut();
    let l = root.left.take();
    let r = root.right.take();
    let c = (root.val as u8 + 'a' as u8) as char;
    suffix.insert(0, c);
    let mut s1 = suffix.clone();
    let mut s2 = suffix.clone();
    if l.is_none() && r.is_none() {
        set.insert(suffix);
        return
    }
    if l.is_some() {
        dfs(l.unwrap(), s1, set);
    }
    if r.is_some() {
        dfs(r.unwrap(), s2, set);
    }
}

fn small_str<'a>(l: &'a String, r: &'a String) -> &'a String {
    let a: Vec<_> = l.chars().collect();
    let b: Vec<_> = r.chars().collect();
    for i in 0..a.len() {
        if i >= b.len() {
            return r
        }
        if a[i] > b[i] {
            return r
        } else if a[i] < b[i] {
            return l
        }
    }
    l
}
