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
    if let Some(root) = root.as_mut() {
        let mut r = root.borrow_mut();
        r.left = prune(r.left.take());
        r.right = prune(r.right.take());
        if r.left.is_none() && r.right.is_none() && r.val == 0 {
            return None
        }
    }
    root
}
