use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = ans.clone();
        increase(root, &mut cur);
        let mut ans = ans.borrow_mut();
        ans.right.take()
    }
}

fn increase(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut Rc<RefCell<TreeNode>>) {
    if root.is_none() {
        return
    }
    let mut rt = root.unwrap();
    let (left, right) = {
        let mut root = rt.borrow_mut();
        (root.left.take(), root.right.take())
    };
    increase(left, cur);
    cur.borrow_mut().right = Some(rt.clone());
    *cur = rt;
    increase(right, cur);
}
