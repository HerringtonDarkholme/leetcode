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
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode{
                val,
                left: None,
                right: None,
            })))
        }
        let mut r = root.as_mut().unwrap();
        let mut n = r.borrow_mut();
        if n.val < val {
            n.right = Solution::insert_into_bst(n.right.take(), val);
        } else {
            n.left = Solution::insert_into_bst(n.left.take(), val);
        }
        drop(n);
        drop(r);
        root
    }
}
