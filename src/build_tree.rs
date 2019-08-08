use crate::util::tree::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;


use std::collections::HashMap;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mapping: HashMap<_, _> = inorder.iter().enumerate().map(|(i, &n)| (n as usize, i)).collect();
        Solution::get_tree(&inorder, &postorder, 0, 0, inorder.len(), &mapping)
    }

    fn get_tree(inorder: &Vec<i32>, postorder: &Vec<i32>, i_start: usize, p_start: usize, len: usize, mapping: &HashMap<usize, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None
        }
        let r = postorder[p_start + len - 1];
        let root_start_inorder = mapping[&(r as usize)];
        let l_len = root_start_inorder - i_start;
        let r_len = len - 1 - l_len;
        let left = Solution::get_tree(inorder, postorder, i_start, p_start, l_len, mapping);
        let right = Solution::get_tree(inorder, postorder, root_start_inorder + 1, p_start + l_len, r_len, mapping);
        Some(Rc::new(RefCell::new(TreeNode{val: r, left, right})))
    }
}

#[test]
fn test() {
    for i in vec![
        (vec![1, 2], vec![2, 1]),
        (vec![1,2,3,4], vec![2,1,4,3]),
    ] {
        Solution::build_tree(i.0, i.1);
    }
}
