use crate::util::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &mut head;
        while let Some(n) = node.as_mut() {
            let mut next = n.next.take();
            while let Some(next_inner) = next {
                if next_inner.val == n.val {
                    next = next_inner.next;
                } else {
                    n.next = Some(next_inner);
                    break;
                }
            }
            node = &mut n.next;
        }
        head
    }
}
