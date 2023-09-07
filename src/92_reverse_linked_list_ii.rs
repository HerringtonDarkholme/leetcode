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
    pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode{val: 0, next: head}));
        let mut h = &mut head;
        let mut cnt = 0;
        while let Some(mut node) = h.as_mut() {
            cnt += 1;
            if cnt == m {
                reverse(&mut node, n - m);
                break;
            } else {
                h = &mut node.next;
            }
        }
        head.unwrap().next
    }
}

fn reverse(node: &mut Box<ListNode>, mut n: i32) {
    let mut prev = None;
    let mut h = node.next.take();
    let end = h.as_ref().unwrap();
    let end: *const ListNode = &**end;
    let end = unsafe {end as *mut ListNode};
    while n >= 0 {
        let next = h.as_mut();
        let next = next.unwrap().next.take();
        let hh = h.as_mut().unwrap();
        hh.next = prev;
        prev = h;
        h = next;
        n -= 1;
    }
    node.next = prev;
    unsafe {
        (*end).next = h;
    }
}


impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut i = 1;
        let mut reversed_head = &mut head;
        while i < left {
            reversed_head = &mut reversed_head.as_mut().unwrap().next;
            i += 1;
        }
        let mut reversed = reversed_head.take();
        let mut prev = None;
        let last = &reversed.as_ref().unwrap().next as *const Option<Box<ListNode>>;
        let mut curr = &mut reversed;
        while i <= right {
            let c = curr.as_mut().unwrap();
            let next = c.next.take();
            c.next = prev;
            prev = curr.take();
            *curr = next;
            i += 1;
        }
        let tail = curr.take();
        unsafe {
            *(last as *mut Option<Box<ListNode>>) = tail;
        }
        *reversed_head = prev;
        head
    }
}
