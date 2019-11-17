// leetcode 61
pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head
        }
        let mut len = find_len(&head);
        let mut k = (len - (k as usize) % len) % len;
        if k == 0 {
            return head
        }
        let mut tail = head;
        let mut front = &mut tail;
        while k > 1 {
            front = &mut front.as_mut().unwrap().next;
            k -= 1;
            len -= 1;
        }
        let mut head = front.as_mut().unwrap().next.take();
        front = &mut head;
        len -= 1;
        while len > 1 {
            front = &mut front.as_mut().unwrap().next;
            len -= 1;
        }
        front.as_mut().unwrap().next = tail;
        head
    }
}

fn find_len(mut head: &Option<Box<ListNode>>) -> usize {
    let mut len = 0;
    while let Some(h) = head.as_ref() {
        head = &h.next;
        len += 1;
    }
    len
}
