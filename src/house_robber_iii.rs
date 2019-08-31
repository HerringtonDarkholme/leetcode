// 337. House Robber III
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ret = Solution::aux(&root);
        i32::max(ret.0, ret.1)
    }
    fn aux(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = root.as_ref() {
            let node = n.borrow();
            let left = Solution::aux(&node.left);
            let right = Solution::aux(&node.right);
            let with = node.val + left.1 + right.1;
            let without = left.0 + right.0;
            (with.max(without), without)
        } else {
            (0, 0)
        }
    }
}
