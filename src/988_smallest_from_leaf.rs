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
// use std::collections::HashSet;
// impl Solution {
//     pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         let mut set = HashSet::new();
//         if root.is_none() {
//             return String::new()
//         }
//         dfs(root.unwrap(), String::new(), &mut set);
//         let mut str: Vec<_> = set.into_iter().collect();
//         let mut small = str[0].clone();
//         for i in 1..str.len() {
//             small = small_str(&small, &str[i]).clone();
//         }
//         small
//     }
// }
// fn dfs(root: Rc<RefCell<TreeNode>>, mut suffix: String,set: &mut HashSet<String>) {
//     let mut root = root.borrow_mut();
//     let l = root.left.take();
//     let r = root.right.take();
//     let c = (root.val as u8 + 'a' as u8) as char;
//     suffix.insert(0, c);
//     let mut s1 = suffix.clone();
//     let mut s2 = suffix.clone();
//     if l.is_none() && r.is_none() {
//         set.insert(suffix);
//         return
//     }
//     if l.is_some() {
//         dfs(l.unwrap(), s1, set);
//     }
//     if r.is_some() {
//         dfs(r.unwrap(), s2, set);
//     }
// }

// fn small_str<'a>(l: &'a String, r: &'a String) -> &'a String {
//     let a: Vec<_> = l.chars().collect();
//     let b: Vec<_> = r.chars().collect();
//     for i in 0..a.len() {
//         if i >= b.len() {
//             return r
//         }
//         if a[i] > b[i] {
//             return r
//         } else if a[i] < b[i] {
//             return l
//         }
//     }
//     l
// }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        smallest_from_leaf(root, vec![])
            .into_iter()
            .map(|c| (c + b'a') as char)
            .collect()
    }
}

fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>, mut p: Vec<u8>) -> Vec<u8> {
    let mut root = match root {
        Some(r) => r,
        None => return p
    };
    let mut root = root.borrow_mut();
    p.insert(0, root.val as u8);
    let left = root.left.take();
    let right = root.right.take();
    let mut l_vec = smallest_from_leaf(left, p.clone());
    let mut r_vec = smallest_from_leaf(right, p.clone());
    if l_vec.len() == p.len() {
        r_vec
    } else if r_vec.len() == p.len() {
        l_vec
    } else {
        l_vec.min(r_vec)
    }
}


use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut max = VecDeque::new();
        let mut p = VecDeque::new();
        s(root, &mut p, &mut max);
        max.into_iter().map(|c| (b'a' + c as u8) as char).collect()
    }
}

fn s(root: Option<Rc<RefCell<TreeNode>>>, p: &mut VecDeque<i32>, max: &mut VecDeque<i32>) {
    let Some(root) = root else { return };
    let mut root = root.borrow_mut();
    p.push_front(root.val);
    if root.left.is_none() && root.right.is_none() {
        if p < max || max.is_empty() { *max = p.clone(); }
    }
    s(root.left.take(), p, max);
    s(root.right.take(), p, max);
    p.pop_front();
}
