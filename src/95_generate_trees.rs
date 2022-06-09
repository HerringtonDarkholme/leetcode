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
use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        gen_trees(1, n)
    }
}
fn gen_node(val: i32) -> Node {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn gen_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }
    if start == end {
        return vec![gen_node(start)];
    }
    let mut ret = vec![];
    for i in start..=end {
        let lefts = gen_trees(start, i - 1);
        let rights = gen_trees(i + 1, end);
        for l in &lefts {
            for r in &rights {
                let mut node = TreeNode::new(i);
                node.left = l.as_ref().map(|c| c.clone());
                node.right = r.as_ref().map(|c| c.clone());
                ret.push(Some(Rc::new(RefCell::new(node))));
            }
        }
    }
    ret
}
