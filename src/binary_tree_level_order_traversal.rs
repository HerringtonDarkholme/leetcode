use crate::util::tree::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        Solution::level_order_recurse(&root, 0, &mut ret);
        ret
    }

    fn level_order_recurse(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, v: &mut Vec<Vec<i32>>) {
        if node.is_none() {
            return
        }
        let node = node.as_ref().unwrap().borrow();
        let val = node.val;
        if v.len() > level {
            v[level].push(val);
        } else {
            v.push(vec![val]);
        }
        Solution::level_order_recurse(&node.left, level + 1, v);
        Solution::level_order_recurse(&node.right, level + 1, v);
    }
}
