/**
adapt from javascript code
var partition = function(head, x) {
    if (!head) {
        return head
    }
    let h = head
    let smallHead = null
    let small = null
    let largeHead = null
    let large = null
    while (h) {
        if (h.val >= x) {
            if (large) {
                large.next = h
            } else {
                largeHead = h
            }
            large = h
        } else {
            if (small) {
               small.next = h
            } else {
                smallHead = h
            }
            small = h

        }
        h = h.next
    }
    if (small) small.next = largeHead
    if (large) large.next = null
    return smallHead || largeHead
};
 */
use crate::util::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less = None;
        let mut greater = None;
        let mut l = &mut less;
        let mut g = &mut greater;
        while let Some(mut h) = head {
            head = h.next.take();
            if h.val < x {
                *l = Some(h);
                l = &mut l.as_mut().unwrap().next;
            } else {
                *g = Some(h);
                g = &mut g.as_mut().unwrap().next;
            }
        }
        *l = greater;
        less
    }
}
