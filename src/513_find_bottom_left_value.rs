use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = (-1, 0);
        find(root, 0, &mut ret);
        ret.1
    }
}

fn find(root: Option<Rc<RefCell<TreeNode>>>, level: i32, ret: &mut (i32, i32)) {
    let root = match root {
        Some(r) => r,
        None => return,
    };
    let mut rt = root.borrow_mut();
    if level > ret.0 {
        *ret = (level, rt.val);
    }
    find(rt.left.take(), level + 1, ret);
    find(rt.right.take(), level + 1, ret);
}
