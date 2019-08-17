pub struct Solution;
use crate::util::linked_list::ListNode;

use std::collections::{BinaryHeap};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    val: i32,
    id: usize,
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for (k, node) in lists.iter().enumerate() {
            if let Some(b) = node {
                heap.push(Node{ val: -b.val, id: k})
            }
        }
        let mut head = None;
        let mut current = &mut head;
        while let Some(Node{val, id}) = heap.pop() {
            current.replace(Box::new(ListNode{val: -val, next: None}));
            current = &mut current.as_mut().unwrap().next;
            let next = lists[id].as_mut().unwrap().next.take();
            if let Some(n) = next.as_ref() {
                heap.push(Node{val: -n.val, id});
            }
            lists[id] = next;
        }
        head
    }
}
