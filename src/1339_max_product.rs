const M: i64 = 1_000_000_007;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_product(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let s = sum_node(&mut root) as i64;
        let r = max_product(&root, s) % M;
        r as i32
    }
}

fn sum_node(root: &mut Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0
    }
    let mut root = root.as_mut().unwrap();
    let mut root = root.borrow_mut();
    let l = sum_node(&mut root.left);
    let r = sum_node(&mut root.right);
    root.val += l + r;
    root.val
}

fn max_product(root: &Option<Rc<RefCell<TreeNode>>>, parent: i64) -> i64 {
    if root.is_none() {
        return 0
    }
    let root = root.as_ref().unwrap();
    let root = root.borrow();
    let v = root.val as i64;
    let m = v * (parent - v);
    let left = max_product(&root.left, parent);
    let right = max_product(&root.right, parent);
    left.max(right).max(m)
}
