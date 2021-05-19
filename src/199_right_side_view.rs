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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        right_side_view(root, 0, &mut ret);
        ret
    }
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>, level: usize, ret: &mut Vec<i32>) {
    if root.is_none() {
        return
    }
    let root = root.unwrap();
    let mut root = root.borrow_mut();
    if level >= ret.len() {
        ret.push(root.val);
    }
    right_side_view(root.right.take(), level + 1, ret);
    right_side_view(root.left.take(), level + 1, ret);
}
