use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if root.is_none() {
            return vec![]
        }
        let root = root.unwrap();
        let mut rt = root.borrow_mut();
        let left = rt.left.take();
        let mut l = Solution::binary_tree_paths(left);
        let right = rt.right.take();
        let r = Solution::binary_tree_paths(right);
        l.extend_from_slice(&r);
        if l.is_empty() {
            vec![format!("{}", rt.val)]
        } else {
            l.into_iter().map(|v| format!("{}->{}", rt.val, v)).collect()
        }
    }
}
