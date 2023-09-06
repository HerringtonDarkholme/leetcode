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
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut size = 0;
        let mut node = head.as_ref();
        while let Some(n) = node {
            size += 1;
            node = n.next.as_ref();
        }
        let per_group = size / k;
        let mut extra = size % k;
        let mut count = 0;
        let mut ret = vec![];
        let mut node = &mut None;
        while let Some(mut h) = head {
            head = h.next.take();
            if count == 0 {
                ret.push(Some(h));
                node = &mut ret.last_mut().unwrap().as_mut().unwrap().next;
            } else {
                *node = Some(h);
                node = &mut node.as_mut().unwrap().next;
            }
            count += 1;
            if count == if extra > 0 { 1 } else { 0 } + per_group {
                count = 0;
                extra -= 1;
            }
        }
        while ret.len() < k as usize {
            ret.push(None);
        }
        ret
    }
}
