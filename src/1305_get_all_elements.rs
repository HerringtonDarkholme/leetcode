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
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn get_all_elements(root1: Tree, root2: Tree) -> Vec<i32> {
        let mut v1 = vec![];
        let mut v2 = vec![];
        get_all(root1, &mut v1);
        get_all(root2, &mut v2);
        v1.extend(v2);
        v1.sort();
        v1
    }
}

fn get_all(r: Tree, ret: &mut Vec<i32>) {
    if r.is_none() {
        return;
    }
    let mut r = r.unwrap();
    let mut r = r.borrow_mut();
    let left = r.left.take();
    let right = r.right.take();
    get_all(left, ret);
    ret.push(r.val);
    get_all(right, ret);
}
