use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(cur_mut) = cur.as_mut() {
            let mut next = match cur_mut.next.take() {
                Some(next) => next,
                None => break,
            };
            cur_mut.next = next.next.take();
            next.next = cur.take();
            *cur = Some(next);
            let cur_mut = cur.as_mut()?;
            let next_mut = cur_mut.next.as_mut()?;
            cur = &mut next_mut.next;
        }
        head
    }


}
