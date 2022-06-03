// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        let mut ret = vec![];
        distance_k(&root, &target, k, &mut ret);
        ret
    }
}

fn distance_k(root: &Option<Rc<RefCell<TreeNode>>>, target: &Option<Rc<RefCell<TreeNode>>>, k: i32, ret: &mut Vec<i32>) -> i32 {
    if root.is_none() {
        return -1;
    }
    if root == target {
        count_distance(root, k, ret);
        return 0;
    }
    let root = root.as_ref().unwrap();
    let root = root.borrow();
    let left_distance = distance_k(&root.left, target, k, ret);
    if left_distance >= 0 {
        if left_distance + 1 == k {
            ret.push(root.val);
        }
        if k >= left_distance + 2 {
            count_distance(&root.right, k - left_distance - 2, ret);
        }
        return left_distance + 1;
    }
    let right_distance = distance_k(&root.right, target, k, ret);
    if right_distance >= 0 {
        if right_distance + 1 == k {
            ret.push(root.val);
        }
        if k >= right_distance + 2 {
            count_distance(&root.left, k - right_distance - 2, ret);
        }
        return right_distance + 1;
    }
    return -1;

}

fn count_distance(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, ret: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let root = root.as_ref().unwrap();
    let mut root = root.borrow();
    if k == 0 {
        ret.push(root.val);
        return;
    }
    count_distance(&root.left, k - 1, ret);
    count_distance(&root.right, k - 1, ret);
}
