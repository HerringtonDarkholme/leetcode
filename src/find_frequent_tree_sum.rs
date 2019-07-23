pub struct Solution;
use crate::util::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashMap;
impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut hash = HashMap::new();
        Solution::find_sum_aux(&root, &mut hash);
        let mut max = 0;
        let mut ret = vec![];
        for (&k, &v) in hash.iter() {
            if v > max {
                max = v;
                ret = vec![k];
            } else if v == max {
                ret.push(k);
            }
        }
        ret
    }

    fn find_sum_aux(root: &Option<Rc<RefCell<TreeNode>>>, hash: &mut HashMap<i32, usize>) -> i32 {
        if let Some(root) = root {
            let tree = root.borrow();
            let l = Solution::find_sum_aux(&tree.left, hash);
            let r = Solution::find_sum_aux(&tree.right, hash);
            let s = l + r + tree.val;
            *hash.entry(s).or_insert(0) += 1;
            s
        } else {
            0
        }
    }
}
