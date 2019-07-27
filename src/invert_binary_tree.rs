use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        if let Some(r) = root.as_mut() {
            let mut rr = r.borrow_mut();
            let left = rr.left.take();
            let right = rr.right.take();
            let new_left = Solution::invert_tree(left);
            let new_right = Solution::invert_tree(right);
            rr.left = new_right;
            rr.right = new_left;
        }
        root
    }
}
