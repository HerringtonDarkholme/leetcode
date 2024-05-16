use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        eval(root.unwrap())
    }
}

fn eval(root: Rc<RefCell<TreeNode>>) -> bool {
    let mut root = root.borrow_mut();
    let v = match root.val {
        0 => return false,
        1 => return true,
        v => v,
    };
    let l = eval(root.left.take().unwrap());
    let r = eval(root.right.take().unwrap());
    if v == 2 {
        l || r 
    } else {
        l && r
    }
}
