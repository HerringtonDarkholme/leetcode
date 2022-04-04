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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut i = 1;
        let mut front = &head;
        let mut prev = &head;
        let mut last = &head;
        while front.is_some() {
            front = &front.as_ref().unwrap().next;
            if i < k {
                prev = &prev.as_ref().unwrap().next;
            }
            if i > k {
                last = &last.as_ref().unwrap().next;
            }
            i += 1;
        }
        let mut prev = (
            unsafe { &mut  *(prev.as_ref().unwrap() as *const Box<ListNode> as *mut Box<ListNode>) }
        ) as &mut Box<ListNode>;
        let mut last = (
            unsafe { &mut *(last.as_ref().unwrap() as *const Box<ListNode> as *mut Box<ListNode>) }
        ) as &mut Box<ListNode>;
        std::mem::swap(&mut prev.val, &mut last.val);
        head
    }
}
