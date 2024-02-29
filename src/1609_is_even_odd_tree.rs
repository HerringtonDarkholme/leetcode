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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        bfs(root)
    }
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut nodes = vec![root];
    let mut level = 0;
    while !nodes.is_empty() {
        let mut next = vec![];
        let mut last = 0;
        for node in nodes {
            let Some(node) = node else { continue; };
            let mut node = node.borrow_mut();
            if node.val % 2 == level % 2 {
                return false;
            }
            if last == 0 {
                last = node.val;
            } else if level % 2 == 0 {
                if last >= node.val { return false; }
            } else {
                if last <= node.val { return false; }
            }
            last = node.val;
            next.push(node.left.take());
            next.push(node.right.take());
        }
        level += 1;
        nodes = next;
    }
    true
}
