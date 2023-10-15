use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(mut p), Some(mut q)) => {
                let mut p = p.borrow_mut();
                let mut q = q.borrow_mut();
                p.val == q.val && 
                    Solution::is_same_tree(p.left.take(), q.left.take()) && 
                    Solution::is_same_tree(p.right.take(), q.right.take())
            },
            _ => false,
        }
    }
}
