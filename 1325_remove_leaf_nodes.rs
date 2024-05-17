use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(mut root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        remove(&mut root, target);
        root
    }
}

fn remove(node: &mut Option<Rc<RefCell<TreeNode>>>, target: i32) {
    let Some(n) = node else { return; };
    let mut n = n.borrow_mut();
    remove(&mut n.left, target);
    remove(&mut n.right, target);
    let need = n.left.is_none() && n.right.is_none() && n.val == target;
    drop(n); // need drop reference
    if need {
        *node = None;
    }
}
