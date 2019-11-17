pub struct Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        aux(&root1, &root2)
    }
}

fn aux(r1: &Option<Rc<RefCell<TreeNode>>>, r2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if r1.is_none() {
        r2.is_none()
    } else if r2.is_none() {
        false
    } else {
        let r1 = r1.as_ref().unwrap();
        let r2 = r2.as_ref().unwrap();
        let n1 = r1.borrow();
        let n2 = r2.borrow();
        if n1.val != n2.val {
            return false
        }
        aux(&n1.left, &n2.left) && aux(&n1.right, &n2.right) ||
        aux(&n1.left, &n2.right) && aux(&n1.right, &n2.left)
    }
}
