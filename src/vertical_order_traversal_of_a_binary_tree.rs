// leetcode 987
pub struct Solution;

use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        dfs(root, 0, 0, &mut map);
        map.into_iter().map(|(_, mut v)| {
            v.sort();
            v.into_iter().map(|v| v.1).collect()
        }).collect()
    }
}
fn dfs(root: Option<Rc<RefCell<TreeNode>>>, y: i32, x: i32, map: &mut BTreeMap<i32, Vec<(i32, i32)>>) {
    if root.is_none() {
        return
    }
    let mut root = root.unwrap();
    let mut rt = root.borrow_mut();
    map.entry(x).or_insert(vec![]).push((y, rt.val));
    let l = rt.left.take();
    let r = rt.right.take();
    dfs(l, y+1, x - 1, map);
    dfs(r, y+1, x + 1, map);
}
