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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        zig_zag(root, true).1 - 1
    }
}
fn zig_zag(root: Option<Rc<RefCell<TreeNode>>>, left: bool) -> (i32, i32) {
    if root.is_none() {
        return (0, 1)
    }
    let mut root = root.unwrap();
    let mut rt = root.borrow_mut();
    let (lr, lmax) = zig_zag(rt.left.take(), true);
    let (rl, rmax) = zig_zag(rt.right.take(), false);
    let (l, r) = (1 + lr, 1 + rl);
    let max = l.max(r).max(lmax).max(rmax);
    (if left { r } else { l }, max)
}
