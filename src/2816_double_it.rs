impl Solution {
    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &mut head;

        if curr.as_ref().map_or(0, |n| n.val) > 4 {
            let mut node = ListNode::new(1);
            node.next = curr.take();
            *curr = Some(Box::new(node));
            curr  = &mut curr.as_mut().unwrap().next;
        }
        while let Some(node) = curr {
            if node.next.as_ref().map_or(0, |n| n.val) > 4 {
                node.val = (node.val * 2 + 1) % 10;
            } else {
                node.val = (node.val * 2) % 10;
            }
            curr = &mut node.next;
        }

        head
    }
}
