use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut ret = 0;
            Solution::sum_numbers_aux(&root, 0, &mut ret);
            ret
        } else {
            0
        }
    }

    fn sum_numbers_aux(root: &Rc<RefCell<TreeNode>>, acc: i32, ret: &mut i32) {
        let root = root.borrow();
        if root.left.is_none() && root.right.is_none() {
            *ret += acc * 10 + root.val;
        } else {
            if let Some(left) = root.left.as_ref() {
                Solution::sum_numbers_aux(left, acc * 10 + root.val, ret);
            }
            if let Some(right) = root.right.as_ref() {
                Solution::sum_numbers_aux(right, acc * 10 + root.val, ret);
            }
        }
    }
}
