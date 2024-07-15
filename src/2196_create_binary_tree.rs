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
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        let mut is_child = HashSet::new();
        for desc in descriptions {
            let (p, c, dir) = (desc[0], desc[1], desc[2]);
            is_child.insert(c);
            let mut p = map.entry(p).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(p)))).clone();
            let c = map.entry(c).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(c)))).clone();
            let mut p = p.borrow_mut();
            if dir == 1 {
                p.left = Some(c.clone());
            } else {
                p.right = Some(c.clone());
            }
        }
        for (k, node) in map {
            if is_child.contains(&k) {
                continue;
            }
            return Some(node)
        }
        None
    }
}
