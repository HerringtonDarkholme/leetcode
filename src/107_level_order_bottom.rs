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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut r = vec![];
        aux(root, 0, &mut r);
        r.reverse();
        r
    }
}

fn aux(root: Option<Rc<RefCell<TreeNode>>>, l: usize, s: &mut Vec<Vec<i32>>) {
    if root.is_none() {
        return
    }
    if l >= s.len() {
        s.push(vec![]);
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    s[l].push(root.val);
    aux(root.left.take(), l + 1, s);
    aux(root.right.take(), l + 1, s);
}
