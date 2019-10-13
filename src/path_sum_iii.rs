
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        sum_aux(&root, sum)
    }
}
fn sum_aux(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    if let Some(r) = root.as_ref() {
        let r = r.borrow();
        sum_aux(&r.left, sum) + sum_aux(&r.right, sum) + sum_from(root, sum)
    } else {
        0
    }
}
fn sum_from(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    if let Some(r) = root.as_ref() {
        let r = r.borrow();
        sum_from(&r.left, sum - r.val) +
        sum_from(&r.right, sum - r.val) +
        if r.val == sum { 1 } else { 0 }
    } else {
        0
    }
}
