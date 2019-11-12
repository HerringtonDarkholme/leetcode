use crate::util::tree:TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        aux(root, sum)
    }
}

fn aux(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![]
    }
    let mut root = root.unwrap();
    let mut root = root.borrow_mut();
    let left = root.left.take();
    let right = root.right.take();
    if left.is_none() && right.is_none() {
        return if root.val == sum {
            vec![vec![sum]]
        } else {
            vec![]
        }
    }
    let mut ret = aux(left, sum - root.val);
    ret.append(&mut aux(right, sum - root.val));
    for v in ret.iter_mut() {
        v.insert(0, root.val);
    }
    ret
}
