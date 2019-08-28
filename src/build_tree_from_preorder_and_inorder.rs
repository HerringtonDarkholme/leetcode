use crate::util::tree::TreeNode;
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let ino = inorder.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        let len = preorder.len();
        Solution::build_aux(0, 0, len, &preorder, &ino)
    }
    fn build_aux(pi: usize, ii: usize, len: usize, pre: &[i32], ino: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None
        }
        if len == 1 {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: pre[pi],
                left: None, right: None,
            })))
        }
        let root_val = pre[pi];
        let root_ii = ino[&root_val];
        let l_len = root_ii - ii;
        let l_pi = pi + 1;
        let l_ii = ii;
        let r_len = len - 1 - l_len;
        let r_pi = pi + 1 + l_len;
        let r_ii = root_ii + 1;
        let root = TreeNode {
            val: root_val,
            left: Solution::build_aux(l_pi, l_ii, l_len, pre, ino),
            right: Solution::build_aux(r_pi, r_ii, r_len, pre, ino),
        };
        Some(Rc::new(RefCell::new(root)))
    }
}
