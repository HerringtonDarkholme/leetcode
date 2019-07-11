use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root.as_ref() {
            Solution::recurse(root).1
        } else {
            0
        }
    }
    fn recurse(root: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
        use std::cmp::max;
        let root = root.borrow();
        let (l_path, l_max) = if let Some(left) = root.left.as_ref() {
            Solution::recurse(left)
        } else {
            (0, i32::min_value())
        };
        let (r_path, r_max) = if let Some(right) = root.right.as_ref() {
            Solution::recurse(right)
        } else {
            (0, i32::min_value())
        };
        let path = max(max(l_path, r_path) + root.val, 0);
        let max = max(l_path + r_path + root.val, max(l_max, r_max));
        (path, max)
    }
}
