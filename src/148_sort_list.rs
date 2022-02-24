impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        merge_sort(head)
    }
}
type Node = Option<Box<ListNode>>;
fn merge_sort(head: Node) -> Node {
    if head.is_none() {
        return None;
    }
    let (first, second) = split(head);
    let first = merge_sort(first);
    let second = merge_sort(second);
    merge(first, second)
}

fn split(head: Node) -> (Node, Node) {
    debug_assert!(head.is_some());
    let mut slow = &head;
    let mut fast = &head;
    while fast.as_ref().unwrap().next.is_some()
        && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
    {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    let tail =
        unsafe { &mut *(slow.as_ref().unwrap() as *const Box<ListNode> as *mut Box<ListNode>) };
    let second = tail.next.take();
    (head, second)
}
fn merge(mut first: Node, mut second: Node) -> Node {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    if first.is_none() || second.is_none() {
        return first.or(second);
    }
    let mut cur = dummy.as_mut().unwrap();
    while first.is_some() && second.is_some() {
        let f = first.as_ref().unwrap().val;
        let s = second.as_ref().unwrap().val;
        if f < s {
            cur.next = first;
            cur = cur.next.as_mut().unwrap();
            first = cur.next.take();
        } else {
            cur.next = second;
            cur = cur.next.as_mut().unwrap();
            second = cur.next.take();
        }
    }
    if first.is_some() {
        cur.next = first;
    } else {
        cur.next = second;
    }
    cur.next.take()
}
