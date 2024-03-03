// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut h = &head;
        let mut p = &head;
        let mut r = head.as_ref().map(|n| &n.next).unwrap_or(&None);
        while h.is_some() && n > -1 {
            h = &h.as_ref()?.next;
            n -= 1;
        }
        if n > -1 { return head.map(|n| n.next).flatten(); }
        while h.is_some() {
            h = &h.as_ref()?.next;
            p = &p.as_ref()?.next;
            r = &r.as_ref()?.next;
        }
        if p.is_none() { return head.map(|n| n.next).flatten(); }
        let mut p = cast(p);
        let mut r = cast(r);
        p.as_mut()?.next = r.as_mut().map(|r| r.next.take()).flatten();
        head
    }
}

#[allow(invalid_reference_casting)]
fn cast<T>(p: &T) -> &mut T {
    unsafe { &mut *(p as *const T as *mut T) as &mut T }
}
