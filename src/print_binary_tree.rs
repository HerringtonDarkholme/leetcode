use crate::util::tree::TreeNode;
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Copy, Clone)]
struct Info {
    pos: usize,
    val: i32,
}

impl Solution {
    pub fn print_tree(root: Node) -> Vec<Vec<String>> {
        let mut stack = vec![vec![]; 10];
        let max_height = Solution::collect(&root, 0, 0, &mut stack);
        let mut ret = vec![];
        let len = usize::pow(2, max_height as u32) - 1;
        for i in 0..max_height {
            let mut v = vec!["".to_owned(); len];
            let interval = usize::pow(2, (max_height - i) as u32);
            for info in stack[i].iter() {
                let p = info.pos;
                v[interval * p + interval / 2 - 1] = format!("{}", info.val);
            }
            ret.push(v);
        }
        ret
    }
    fn collect(root: &Node, pos: usize, level: usize, stack: &mut Vec<Vec<Info>>) -> usize {
        if let Some(r) = root {
            let r = r.borrow();
            stack[level].push(Info{pos, val: r.val});
            let left = Solution::collect(&r.left, 2*pos, level + 1, stack);
            let right = Solution::collect(&r.right, 2*pos+1, level + 1, stack);
            std::cmp::max(left, right) + 1
        } else {
            0
        }
    }
}
