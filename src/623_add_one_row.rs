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
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut dummy = TreeNode::new(0);
        dummy.left = root;
        let dummy = add_one_row(Some(Rc::new(RefCell::new(dummy))), val, depth);
        let dummy = dummy.unwrap();
        let mut dummy = dummy.borrow_mut();
        dummy.left.take()
    }
}
pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = match root {
        Some(r) => r,
        None => return None,
    };
    let mut rt = root.borrow_mut();
    let left = rt.left.take();
    let right = rt.right.take();
    if depth == 1 {
        let mut new_left = TreeNode::new(val);
        new_left.left = left;
        let mut new_right = TreeNode::new(val);
        new_right.right = right;
        rt.left = Some(Rc::new(RefCell::new(new_left)));
        rt.right = Some(Rc::new(RefCell::new(new_right)));
    } else {
        rt.left = add_one_row(left, val, depth - 1);
        rt.right = add_one_row(right, val, depth - 1);
    }
    drop(rt);
    return Some(root);
}
