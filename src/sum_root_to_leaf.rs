// leetcode 1022
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        dfs(root, 0, &mut sum);
        sum
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: i32, sum: &mut i32) {
    if root.is_none() {
        return
    }
    let root = root.unwrap();
    let mut r = root.borrow_mut();
    let prev = prev * 2 + r.val;
    let left = r.left.take();
    let right = r.right.take();
    if left.is_none() && right.is_none() {
        *sum += prev;
    } else {
        dfs(left, prev, sum);
        dfs(right, prev, sum);
    }
}
