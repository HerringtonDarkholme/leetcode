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
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        aux(root, p, q).unwrap()
    }
}

fn aux(root: Tree, p: i32, q: i32) -> Result<Tree, i32> {
    if root.is_none() {
        return Err(0)
    }
    let mut root = root.unwrap();
    let (m, l, r) = {
        let mut rt = root.borrow_mut();
        let m = if rt.val == p || rt.val == q {
            1
        } else {
            0
        };
        let l = rt.left.take();
        let r = rt.right.take();
        (m, l, r)
    };
    match (aux(l, p, q), aux(r, p, q)) {
        (Err(1), Err(1)) =>  {
            Ok(Some(root))
        },
        (Err(a), Err(b)) => {
            let count = a + b + m;
            if count == 2 {
                Ok(Some(root))
            } else {
                Err(count)
            }
        }
        (a, b) => a.or(b),
    }
}
