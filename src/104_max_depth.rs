use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max(root)
    }
}
fn max(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();
    let mut root = root.borrow_mut();
    let left = root.left.take();
    let right = root.right.take();
    max(left).max(max(right)) + 1
}
