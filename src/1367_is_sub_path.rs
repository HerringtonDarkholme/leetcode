
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_sub_path(&head, &root)
    }
}

fn is_sub_path(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let Some(h) = head.as_ref() else { return true; };
    let Some(root) = root.as_ref() else { return false; };
    let root = root.borrow();
    if h.val == root.val && (cons(&h.next, &root.left) || cons(&h.next, &root.right)) {
        return true
    } else {
        return is_sub_path(head, &root.left) || is_sub_path(head, &root.right)
    }

    
}
fn cons(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let Some(h) = head.as_ref() else { return true; };
    let Some(root) = root.as_ref() else { return false; };
    let root = root.borrow();
    if h.val != root.val {
        return false 
    }
    cons(&h.next, &root.left) || cons(&h.next, &root.right)
}
