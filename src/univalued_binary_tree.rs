pub struct Solution;

use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            true
        } else {
            let root = root.unwrap();
            let mut root = root.borrow_mut();
            let v = root.val;
            let left = root.left.take();
            let right = root.right.take();
            aux(left, v) && aux(right, v)
        }
    }
}
fn aux(node: Option<Rc<RefCell<TreeNode>>>, v: i32) -> bool {
    if let Some(node) = node {
        let mut n = node.borrow_mut();
        if n.val != v {
            return false
        }
        let left = n.left.take();
        let right = n.right.take();
        aux(left, v) && aux(right, v)
    } else {
        true
    }
}
