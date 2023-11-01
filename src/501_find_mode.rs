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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let (mut last, mut max, mut cnt) = (0, 0, 0);
        let mut ret = vec![];
        dfs(root, &mut last, &mut max, &mut ret, &mut cnt);
        ret
    }
}
// use inorder to find max cnt in sorted seq
fn dfs(
    root: Option<Rc<RefCell<TreeNode>>>, last: &mut i32,
    max: &mut i32, ret: &mut Vec<i32>, cnt: &mut i32,
) {
    if let Some(mut root) = root {
        let mut rt = root.borrow_mut();
        dfs(rt.left.take(), last, max, ret, cnt);
        if rt.val == *last {
            *cnt += 1;
        } else {
            *cnt = 1;
        }
        if *cnt > *max {
            *max = *cnt;
            *ret = vec![rt.val];
        } else if *cnt == *max {
            ret.push(rt.val);
        }
        *last = rt.val;
        dfs(rt.right.take(), last, max, ret, cnt);
    }
}
