// leetcode 101
use std::rc::Rc;
use crate::util::tree:TreeNode;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true
        }
        let mut root = root.unwrap();
        let mut root = root.borrow_mut();
        let l = root.left.take();
        let r = root.right.take();
        is_sym(l, r)
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn is_sym(left: Node, right: Node) -> bool {
    if left.is_none() {
        return right.is_none()
    } else if right.is_none() {
        return false
    }
    let mut left = left.unwrap();
    let mut right = right.unwrap();
    let mut left = left.borrow_mut();
    let mut right = right.borrow_mut();
    if left.val != right.val {
        return false
    }
    let ll = left.left.take();
    let lr = left.right.take();
    let rl = right.left.take();
    let rr = right.right.take();
    is_sym(ll, rr) && is_sym(lr, rl)
}
