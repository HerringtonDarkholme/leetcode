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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        find(root, k, &mut set)
    }
}

fn find(root: Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
    if root.is_none() {
        return false;
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    if set.contains(&(k - root.val)) {
        return true;
    }
    set.insert(root.val);
    let left = root.left.take();
    let right = root.right.take();
    find(left, k, set) || find(right, k, set)
}
