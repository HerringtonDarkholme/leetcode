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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals = [0; 10];
        aux(root.unwrap(), &mut vals)
    }
}
fn aux(root: Rc<RefCell<TreeNode>>, vals: &mut [i32]) -> i32 {
    let mut rt = root.borrow_mut();
    vals[rt.val as usize] += 1; // dfs
    let left = rt.left.take();
    let right = rt.right.take();
    let ret = if left.is_none() && right.is_none() {
        if is_pal(vals) { 1 } else { 0 }
    } else if left.is_none() {
        aux(right.unwrap(), vals)
    } else if right.is_none() {
        aux(left.unwrap(), vals)
    } else {
        aux(left.unwrap(), vals) + aux(right.unwrap(), vals)
    };
    vals[rt.val as usize] -= 1; // put back
    ret
}

fn is_pal(vals: &[i32]) -> bool {
    let mut has_odd = false;
    for &v in vals {
        if v % 2 == 0 {
            continue;
        }
        if has_odd {
            return false;
        }
        has_odd = true;
    }
    true
}
