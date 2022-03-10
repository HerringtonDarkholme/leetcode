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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1 = reverse(l1);
        let l2 = reverse(l2);
        let l = add(l1, l2, 0);
        reverse(l)
    }
}

fn reverse(mut l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l.is_none() {
        return None
    }
    let mut n = l.unwrap();
    let next = reverse(n.next.take());
    n.next = next;
    Some(n)
}

fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(n1), Some(n2)) => {
            let val = n1.val + n2.val + carry;
            Some(Box::new(ListNode {
                val: val % 10,
                next: add(n1.next, n2.next, val / 10),
            }))
        },
        (None, Some(n2)) => {
            if carry == 0 {
                return Some(n2)
            }
            let val = n2.val + carry;
            Some(Box::new(ListNode {
                val: val % 10,
                next: add(None, n2.next, val / 10),
            }))
        },
        (Some(n1), None) => {
            if carry == 0 {
                return Some(n1)
            }
            let val = n1.val + carry;
            Some(Box::new(ListNode {
                val: val % 10,
                next: add(n1.next, None, val / 10),
            }))
        },
        (None, None) => {
            if carry == 1 {
                Some(Box::new(ListNode {
                    val: 1,
                    next: None
                }))
            } else {
                None
            }
        }
    }
}
