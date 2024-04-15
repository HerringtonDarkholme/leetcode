use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut ret = 0;
            Solution::sum_numbers_aux(&root, 0, &mut ret);
            ret
        } else {
            0
        }
    }

    fn sum_numbers_aux(root: &Rc<RefCell<TreeNode>>, acc: i32, ret: &mut i32) {
        let root = root.borrow();
        if root.left.is_none() && root.right.is_none() {
            *ret += acc * 10 + root.val;
        } else {
            if let Some(left) = root.left.as_ref() {
                Solution::sum_numbers_aux(left, acc * 10 + root.val, ret);
            }
            if let Some(right) = root.right.as_ref() {
                Solution::sum_numbers_aux(right, acc * 10 + root.val, ret);
            }
        }
    }
}

/*
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        sum_numbers(root, 0, &mut ret);
        ret
    }
}

fn sum_numbers(node: Option<Rc<RefCell<TreeNode>>>, acc: i32, sum: &mut i32) -> bool {
    let Some(node) = node else { return false; };
    let mut n = node.borrow_mut();
    let acc = acc * 10 + n.val;
    let l = sum_numbers(n.left.take(), acc, sum);
    let r = sum_numbers(n.right.take(), acc, sum);
    if !l && !r { *sum += acc; }
    true
}

*/
