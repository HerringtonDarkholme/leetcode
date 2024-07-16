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
impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let root = find_lca(root, start_value, dest_value);
        let mut s = String::new();
        let start = find_path(root.clone(), start_value, &mut s);
        let mut s = s.chars().map(|_| 'U').collect();
        let end = find_path(root, dest_value, &mut s);
        s
    }
}
fn find_lca(root: Option<Rc<RefCell<TreeNode>>>, s: i32, e: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let Some(node) = root.as_ref() else {
        return root
    };
    let node = node.borrow();
    if node.val == s || node.val == e {
        return root.clone();
    }
    let l = find_lca(node.left.clone(), s, e);
    let r = find_lca(node.right.clone(), s, e);
    if l.is_some() && r.is_some() {
        return root.clone()
    } else if l.is_some() {
        l
    } else {
        r
    }
}
fn find_path(root: Option<Rc<RefCell<TreeNode>>>, t: i32, s: &mut String) -> bool {
    let Some(node) = root.as_ref() else {
        return false;
    };
    let node = node.borrow();
    if node.val == t {
        return true;
    }
    s.push('L');
    if find_path(node.left.clone(), t, s) {
        return true
    }
    s.pop();
    s.push('R');
    if find_path(node.right.clone(), t, s) {
        return true
    }
    s.pop();
    false
}
