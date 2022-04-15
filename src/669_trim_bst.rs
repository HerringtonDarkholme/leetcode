use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn trim_bst(mut root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        trim_bst(&mut root, low, high);
        root
    }
}


fn trim_bst(root: &mut Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) {
    if root.is_none() {
        return;
    }
    let mut rt = root.as_mut().unwrap();
    let mut rt = rt.borrow_mut();
    trim_bst(&mut rt.left, low, high);
    trim_bst(&mut rt.right, low, high);
    let replaced = if rt.val < low {
        rt.right.take()
    } else if rt.val > high {
        rt.left.take()
    } else {
        return
    };
    drop(rt);
    *root = replaced;
}
