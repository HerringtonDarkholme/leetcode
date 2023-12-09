use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = match root {
            Some(r) => r,
            None => return vec![],
        };
        let mut r = r.borrow_mut();
        let mut ret = Self::inorder_traversal(r.left.take());
        ret.push(r.val);
        ret.extend(Self::inorder_traversal(r.right.take()));
        ret
    }
}
