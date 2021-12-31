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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max(root).0
    }
}

fn max(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    if root.is_none() {
        return (0, i32::MIN, i32::MAX)
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    let val = root.val;
    let left = root.left.take();
    let right = root.right.take();
    let (l_diff, l_max, l_min) = max(left);
    let (r_diff, r_max, r_min) = max(right);
    let max = val.max(l_max).max(r_max);
    let min = val.min(l_min).min(r_min);
    let diff = (max - val).abs().max(val - min).abs().max(l_diff).max(r_diff);
    (diff, max, min)
}
