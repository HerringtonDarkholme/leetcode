use crate::util::tree::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        level_order(vec![root])
    }
}

fn level_order(nodes: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Vec<Vec<i32>> {
    if nodes.is_empty() {
        return vec![]
    }
    let mut curr = vec![];
    let mut next = vec![];
    for n in nodes {
        if n.is_none() {
            continue;
        }
        let mut n = n.unwrap();
        let mut n = n.borrow_mut();
        curr.push(n.val);
        next.push(n.left.take());
        next.push(n.right.take());
    }
    let mut ret = level_order(next);
    if !curr.is_empty() {
        ret.insert(0, curr);
    }
    ret
}

/*
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
*/
