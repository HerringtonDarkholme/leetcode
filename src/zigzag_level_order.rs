use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

type OptTree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        zigzag(vec![root], false)
    }
}

fn zigzag(roots: Vec<OptTree>, reverse: bool) -> Vec<Vec<i32>> {
    if roots.is_empty() {
        return vec![]
    }
    let mut next_level = vec![];
    let mut current = vec![];
    for root in roots {
        if let Some(root) = root {
            let mut r = root.borrow_mut();
            current.push(r.val);
            next_level.push(r.left.take());
            next_level.push(r.right.take());
        }
    }
    if reverse {
        current.reverse();
    }
    let mut others = zigzag(next_level, !reverse);
    if !current.is_empty() {
        others.insert(0, current);
    }
    others
}
