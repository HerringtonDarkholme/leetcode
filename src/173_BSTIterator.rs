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
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if let Some(rt) = root {
            Self {
                stack: vec![rt],
            }
        } else {
            Self {
                stack: vec![],
            }
        }
    }
    
    fn next(&mut self) -> i32 {
        loop {
            let n = self.stack.last().unwrap();
            let mut n = n.borrow_mut();
            if let Some(left) = n.left.take() {
                drop(n);
                self.stack.push(left);
                continue;
            }
            let ret = n.val;
            let right = n.right.take();
            drop(n);
            self.stack.pop();
            if let Some(right) = right {
                self.stack.push(right);
            }
            break ret
        }
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty() 
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
