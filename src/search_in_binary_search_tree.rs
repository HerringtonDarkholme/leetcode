pub struct Solution;
use $crate::util::tree:TreeNoe;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let nval = {
                let n = node.borrow_mut();
                n.val
            };
            if nval == val {
                Some(node)
            } else if nval > val {
                let mut n = node.borrow_mut();
                let left = n.left.take();
                Solution::search_bst(left, val)
            } else {
                let mut n = node.borrow_mut();
                let right = n.right.take();
                Solution::search_bst(right, val)
            }
        } else {
            None
        }
    }
}
