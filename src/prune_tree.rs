// leetcode 814
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        prune(root)
    }
}

fn prune(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root
    }

    let root = root.unwrap();
    { // drop borrow_mut
    let mut r = root.borrow_mut();
    let left = prune(r.left.take());
    let right = prune(r.right.take());
    if left.is_none() && right.is_none() && r.val == 0 {
        return None
    }
    r.left = left;
    r.right = right;
    }
    Some(root)
}
