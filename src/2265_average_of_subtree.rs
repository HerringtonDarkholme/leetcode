use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        aux(root).2
    }
}

fn aux(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    let mut root = match root {
        Some(r) => r,
        None => return (0, 0, 0),
    };
    let mut rt = root.borrow_mut();
    let (ls, lc, lr) = aux(rt.left.take());
    let (rs, rc, rr) = aux(rt.right.take());
    let sum = ls + rs + rt.val;
    let cnt = lc + rc + 1;
    let ret = if sum / cnt == rt.val { 1 } else { 0 } + lr + rr;
    (sum, cnt, ret)
}
