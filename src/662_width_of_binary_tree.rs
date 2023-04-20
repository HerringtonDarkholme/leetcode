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
use std::collections::VecDeque;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        span(root).unwrap_or(0)
    }
}

fn span(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    let root = root?;
    let mut nodes = VecDeque::new();
    nodes.push_back((0, root));
    let mut max = 0;
    while !nodes.is_empty() {
        let mut min_i = i64::MAX;
        let mut max_i = i64::MIN;
        let len = nodes.len();
        for _ in 0..len {
            let (i, node) = nodes.pop_front()?;
            min_i = min_i.min(i);
            max_i = max_i.max(i);
            let mut node = node.borrow_mut();
            if let Some(l) = node.left.take() {
                nodes.push_back((2 * i, l));
            }
            if let Some(r) = node.right.take() {
                nodes.push_back((2 * i + 1, r));
            }
        }
        max = max.max(max_i - min_i);
    }
    Some(max as i32 + 1)
}
/*
    
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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        width(root)
    }
}

fn width(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = vec![(0, root.unwrap())];
    let mut next = vec![];
    let mut ret = 0;
    while !queue.is_empty() {
        let max = queue.last().unwrap().0 - queue.first().unwrap().0 + 1;
        ret = ret.max(max);
        for (i, mut node) in queue {
            let mut n = node.borrow_mut();
            if let Some(left) = n.left.take() {
                next.push((2 * i, left));
            }
            if let Some(right) = n.right.take() {
                next.push((2 * i + 1, right));
            }
        }
        queue = next;
        next = vec![];
    }
    ret
}

// parent index = n
// left_child index = 2 * n 
// right_child index = 2 * n + 1
// max width = queue[0] 
    
    
    */ 
