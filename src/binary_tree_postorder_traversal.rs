use crate::util::tree::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        Solution::aux(&root, &mut v);
        v
    }
    fn aux(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            Solution::aux(&root.left, v);
            Solution::aux(&root.right, v);
            v.push(root.val);
        }
    }
}
