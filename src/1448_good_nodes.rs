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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        good_nodes(root, i32::MIN)
    }
}

fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>, g: i32) -> i32 {
    if root.is_none() {
        return 0
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    let m = g.max(root.val);
    let l = good_nodes(root.left.take(), m);
    let r = good_nodes(root.right.take(), m);
    if m == root.val {
        l + r + 1
    } else {
        l + r
    }

}
