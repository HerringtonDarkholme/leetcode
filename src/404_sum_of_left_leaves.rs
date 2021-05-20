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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_of_left_leaves(root, false)   
    }
}

fn sum_of_left_leaves(n: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
    if n.is_none() {
        return 0
    }
    let mut n = n.unwrap();
    let mut n = n.borrow_mut();
    if n.left.is_none() && n.right.is_none() {
        if is_left { n.val } else { 0 }
    } else {
        sum_of_left_leaves(n.left.take(), true) + sum_of_left_leaves(n.right.take(), false)
    }
}
