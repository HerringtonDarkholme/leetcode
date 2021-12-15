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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        sort(head)
    }
}

fn sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ret: Option<Box<ListNode>> = None;
    let mut h = unsafe { &mut ret as *mut Option<Box<ListNode>>};
    let mut remain = head;
    while let Some(mut node) = remain {
        let mut front = h;
        let mut last = None;
        while let Some(f) = (unsafe {&mut *front}).as_mut() {
            if node.val > f.val {
                last = Some(f as *mut Box<ListNode>);
                front = &mut f.next;
                continue;
            }
            break;
        }
        remain = node.next;
        node.next = (unsafe {&mut *front}).take();
        if let Some(l) = last {
            unsafe { (*l).next = Some(node) };
        } else {
            
            unsafe {
            *h = Some(node);        
            }
        }
    }
    ret
}
