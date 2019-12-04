pub struct Solution;
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

type Node = Option<Box<ListNode>>;
impl Solution {
    pub fn add_two_numbers(l1: Node, l2: Node) -> Option<Box<ListNode>> {
        let len1 = count_len(&l1);
        let len2 = count_len(&l2);
        let (l1, l2, len1, len2) = if len1 > len2 {
            (l1, l2, len1, len2)
        } else {
            (l2, l1, len2, len1)
        };
        let l2 = make_list(len1 - len2, l2, 0);
        let (carry, node) = add_recur(&l1, &l2);
        if carry == 1 {
            make_list(1, node, 1)
        } else {
            node
        }
    }
}

fn count_len(mut l: &Node) -> usize {
    let mut i = 0;
    while l.is_some() {
        i += 1;
        l = &l.as_ref().unwrap().next;
    }
    i
}

fn make_list(len: usize, tail: Node, val: i32) -> Node {
    if len == 0 {
        tail
    } else {
        let mut node = ListNode::new(val);
        node.next = tail;
        make_list(len - 1, Some(Box::new(node)), val)
    }
}

fn add_recur(l1: &Node, l2: &Node) -> (i32, Node) {
    if l1.is_none() && l2.is_none() {
        return (0, None)
    }
    let l1 = l1.as_ref().unwrap();
    let l2 = l2.as_ref().unwrap();
    let (carry, tail) = add_recur(&l1.next, &l2.next);
    let sum = l1.val + l2.val + carry;
    if sum >= 10 {
        (1, make_list(1, tail, sum - 10))
    } else {
        (0, make_list(1, tail, sum))
    }
}
