// leetcode 101
use std::rc::Rc;
use crate::util::tree:TreeNode;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_sym(&root, &root)
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn is_sym(left: &Node, right: &Node) -> bool {
    if left.is_none() {
        return right.is_none()
    } else if right.is_none() {
        return false
    }
    let mut left = left.as_ref().unwrap();
    let mut right = right.as_ref().unwrap();
    let mut left = left.borrow();
    let mut right = right.borrow();
    if left.val != right.val {
        return false
    }
    let ll = &left.left;
    let lr = &left.right;
    let rl = &right.left;
    let rr = &right.right;
    is_sym(ll, rr) && is_sym(lr, rl)
}
