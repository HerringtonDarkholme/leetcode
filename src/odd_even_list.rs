use crate::util::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd = None;
        let mut even = None;
        let mut odd_head = &mut odd;
        let mut even_head = &mut even;
        let mut i = 1;
        while let Some(mut h) = head {
            let next = h.next.take();
            if i % 2 == 1 {
                odd_head.replace(h);
                odd_head = &mut odd_head.as_mut().unwrap().next;
            } else {
                even_head.replace(h);
                even_head = &mut even_head.as_mut().unwrap().next;
            }
            i += 1;
            head = next;
        }
        *odd_head = even;
        odd
    }
}
