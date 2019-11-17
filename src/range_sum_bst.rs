// leetcode 938
use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if let Some(root) = root {
            let mut rt = root.borrow_mut();
            let mut sum = 0;
            if rt.val >= l && rt.val <= r {
                sum += rt.val;
            }
            if rt.val > l {
                sum += Solution::range_sum_bst(rt.left.take(), l, r);
            }
            if rt.val < r {
                sum += Solution::range_sum_bst(rt.right.take(), l, r);
            }
            sum
        } else {
            0
        }
    }
}
