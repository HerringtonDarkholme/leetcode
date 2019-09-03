use crate::util::tree::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }
        let mut stack = VecDeque::new();
        stack.push_back((root, 1));
        while !stack.is_empty() {
            let (mut node, depth) = stack.pop_front().unwrap();
            let mut node = node.unwrap();
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            if left.is_none() && right.is_none() {
                return depth
            }
            if left.is_some() {
                stack.push_back((left, depth + 1));
            }
            if right.is_some() {
                stack.push_back((right, depth + 1));
            }
        }
        0
    }
}
