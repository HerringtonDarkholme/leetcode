use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        aux(root, k, 0).unwrap()
    }
}
fn aux(root: Option<Rc<RefCell<TreeNode>>>, k: i32, prev: i32) -> Result<i32, i32> {
    if root.is_none() {
        return Err(prev);
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    let left = root.left.take();
    let right = root.right.take();
    match aux(left, k, prev) {
        Err(p) => if p + 1 == k { Ok(root.val) } else { aux(right, k, p + 1) },
        Ok(i) => Ok(i),
    }
}
