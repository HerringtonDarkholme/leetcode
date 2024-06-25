use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        aux(root.as_mut(), 0);
        root
    }
}

fn aux(node: Option<&mut Rc<RefCell<TreeNode>>>, greater: i32) -> i32 {
    let Some(node) = node else { return greater; };
    let mut node = node.borrow_mut();
    let mut g = aux(node.right.as_mut(), greater);
    node.val += g;
    let val = node.val;
    aux(node.left.as_mut(), val)
}
