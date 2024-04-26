use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(r) = root {
            has(r, target_sum)
        } else {
            false
        }
    }
}

fn has(root: Rc<RefCell<TreeNode>>, target: i32) -> bool {
    let mut root = root.borrow_mut();
    if root.left.is_none() && root.right.is_none() {
        return target == root.val;
    }
    if let Some(l) = root.left.take() {
        if has(l, target - root.val) {
            return true
        }
    }
    if let Some(r) = root.right.take() {
        has(r, target - root.val)
    } else {
        false
    }
}
