use crate::util::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn reverse_k_group(mut node: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        None // I don't think rust is capable of doing below... or I don't have time to do
        // it can be done
        // https://github.com/aylei/leetcode-rust/blob/4fb84c1a62264f0dda3a28934d3b12aa7cc49616/src/n0025_reverse_nodes_in_k_group.rs
    }
}

/*
var reverseKGroup = function(node, k) {
    let count = k
    let n = node
    if (!node) {
        return node
    }
    while (n && count > 0) {
        n = n.next
        count -= 1
    }
    if (count !== 0) {
        return node
    }
    let last = n
    let current = node
    while (count < k) {
        let next = current.next
        current.next = last
        last = current
        current = next
        count += 1
    }
    node.next = reverseKGroup(node.next, k)
    return last
};
*/
