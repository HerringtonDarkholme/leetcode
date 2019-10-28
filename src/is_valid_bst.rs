use crate::util::tree::TreeNode;
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        validate(&root, &mut i64::min_value())
    }
}

fn validate(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i64) -> bool {
    if let Some(node) = root.as_ref() {
        let node = node.borrow();
        if validate(&node.left, max) && *max < node.val as i64 {
            *max = node.val as i64;
            validate(&node.right, max)
        } else {
            false
        }
    } else {
        true
    }
}
