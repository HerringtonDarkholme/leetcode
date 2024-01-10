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
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut map = HashMap::new();
        tree_to_graph(root, &mut map);
        compute(map, start)
    }
}

fn tree_to_graph(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, Vec<i32>>) -> Option<i32> {
    let mut root = root?;
    let mut rt = root.borrow_mut();
    let mut children = vec![];
    let val = rt.val;
    if let Some(left) = tree_to_graph(rt.left.take(), map) {
        map.get_mut(&left).unwrap().push(val);
        children.push(left);
    }
    if let Some(right) = tree_to_graph(rt.right.take(), map) {
        map.get_mut(&right).unwrap().push(val);
        children.push(right);
    }
    map.insert(val, children);
    Some(val)
}

fn compute(map: HashMap<i32, Vec<i32>>, start: i32) -> i32 {
    let mut ret = 0;
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut frontier = map.get(&start).unwrap().clone();
    while !frontier.is_empty() {
        let mut next = vec![];
        for n in frontier {
            seen.insert(n);
            for &child in &map[&n] {
                if seen.contains(&child) {
                    continue;
                }
                next.push(child);
            }
        }
        frontier = next;
        ret += 1;
    }
    ret
}
