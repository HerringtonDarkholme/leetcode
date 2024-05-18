use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        aux(root).0
    }
}
fn aux(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let Some(root) = root else { return (0, 0) };
    let mut r = root.borrow_mut();
    let (l_cost, l_val) = aux(r.left.take());
    let (r_cost, r_val) = aux(r.right.take());
    (
        l_cost + r_cost + l_val.abs() + r_val.abs(),
        r.val - 1 + l_val + r_val,
    )
}
