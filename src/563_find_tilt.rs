use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        find_tilt(root).1
    }
}
fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    let (l_val, l_tilt) = find_tilt(root.left.take());
    let (r_val, r_tilt) = find_tilt(root.right.take());
    let diff = (l_val - r_val).abs();
    (root.val + l_val + r_val, diff + l_tilt + r_tilt)
}
