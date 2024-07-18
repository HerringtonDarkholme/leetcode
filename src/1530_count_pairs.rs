use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut ret = 0;
        count_pairs(root, distance, &mut ret);
        ret
    }
}

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, dis: i32, ret: &mut i32) -> Vec<i32> {
    let Some(root) = root else { return vec![] };
    let mut r = root.borrow_mut();
    let l = count_pairs(r.left.take(), dis, ret);
    let r = count_pairs(r.right.take(), dis, ret);
    if l.is_empty() && r.is_empty() { return vec![1]; }
    let mut count = vec![0];
    for (dl, &l_leaf) in l.iter().enumerate() {
        for (dr, &r_leaf) in r.iter().enumerate() {
            if dl + dr + 2 <= dis as usize { *ret += l_leaf * r_leaf; }
        }
        count.push(l_leaf);
    }
    for (r, r_leaf) in r.into_iter().enumerate() {
        if count.len() <= r + 1 {
            count.push(r_leaf);
        } else {
            count[r + 1] += r_leaf;
        }
    }
    count
}
