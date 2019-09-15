use crate::util::tree:TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::aux(root).1
    }
    fn aux(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let (lpath, lmax) = Solution::aux(left);
            let (rpath, rmax) = Solution::aux(right);
            let path = lpath.max(rpath) + 1;
            let max = lmax.max(lpath + rpath + 2).max(rmax);
            (path, max)
        } else {
            (-1, 0)
        }
    }
}
