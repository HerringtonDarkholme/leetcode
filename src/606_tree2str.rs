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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ret = Vec::new();
        if let Some(root) = root {
            tree2str(root, &mut ret);
        }
        String::from_utf8(ret).unwrap()
    }
}
fn tree2str(mut root: Rc<RefCell<TreeNode>>, target: &mut Vec<u8>) {
    let mut root = root.borrow_mut();
    write!(target, "{}", root.val);
    if let Some(left) = root.left.take() {
        write!(target, "(");
        tree2str(left, target);
        write!(target, ")");
    } else if root.right.is_some() {
        write!(target, "()");
    }
    if let Some(right) = root.right.take() {
        write!(target, "(");
        tree2str(right, target);
        write!(target, ")");
    }
}
