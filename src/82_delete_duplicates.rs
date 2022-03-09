impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut prev = &mut ret;
        let mut curr = head;
        while let Some(mut c) = curr {
            if c.next.as_ref().map(|k| k.val != c.val).unwrap_or(true) {
                curr = c.next.take();
                prev.replace(c);
                prev = &mut prev.as_mut().unwrap().next;
                continue;
            }
            let mut next = None;
            while let Some(mut n) = c.next {
                if n.val != c.val {
                    next = Some(n);
                    break;
                } else {
                    c = n;
                }
            }
            curr = next;
        }
        ret
    }
}
