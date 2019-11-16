pub struct Solution;
use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut stack = vec![root];
        while !stack.is_empty() {
            let mut found = -1;
            let mut next = vec![];
            for (i, root) in stack.into_iter().enumerate() {
                if root.is_none() {
                    continue;
                }
                let root = root.unwrap();
                let mut r = root.borrow_mut();
                if r.val == x || r.val == y {
                    if found != -1 {
                        return found / 2 !=  i as i32 / 2
                    }
                    found = i as i32;
                }
                next.push(r.left.take());
                next.push(r.right.take());
            }
            if found != -1 {
                return false
            }
            stack = next;
        }
        false
    }
}
