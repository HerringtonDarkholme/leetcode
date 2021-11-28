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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_sub_recur(root.as_ref(), sub_root.as_ref())
    }
}
type Node<'a> = Option<&'a Rc<RefCell<TreeNode>>>;
fn is_sub_recur(root: Node, sub: Node) -> bool {
    if is_sub(root.clone(), sub.clone()) {
       return true;
    }
    let root = match root {
        Some(r) => r,
        None => return false,
    };
    let root = root.borrow();
    is_sub_recur(root.left.as_ref(), sub.clone()) ||
    is_sub_recur(root.right.as_ref(), sub.clone())
}
fn is_sub(root: Node, sub: Node) -> bool {
    if root.is_none() {
        return sub.is_none();
    }
    if sub.is_none() {
        return false;
    }
    let root = root.unwrap();
    let sub = sub.unwrap();
    let root = root.borrow();
    let sub = sub.borrow();
    if root.val != sub.val {
        return false;
    }
    is_sub(root.left.as_ref(), sub.left.as_ref()) &&
    is_sub(root.right.as_ref(), sub.right.as_ref())
}
