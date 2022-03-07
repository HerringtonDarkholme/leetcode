impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut node = &mut ret;
        loop {
            if l1.is_some() {
                if l2.is_some() {
                    let l1_v = l1.as_ref().unwrap().val;
                    let l2_v = l2.as_ref().unwrap().val;
                    *node = if l1_v < l2_v {
                        let n = l1.as_mut().unwrap().next.take();
                        std::mem::replace(&mut l1, n)
                    } else {
                        let n = l2.as_mut().unwrap().next.take();
                        std::mem::replace(&mut l2, n)
                    };
                    node = &mut node.as_mut().unwrap().next;
                } else {
                    *node = l1;
                    break;
                }
            } else {
                *node = l2;
                break;
            }
        }
        ret
    }
}
/*
impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut node = &mut ret;
        loop {
            match (&l1, &l2) {
                (Some(n1), Some(n2)) => {
                    let n = if n1.val < n2.val {
                        let n = l1.as_mut().unwrap();
                        let next = n.next.take();
                        let n = l1;
                        l1 = next;
                        n
                    } else {
                        let n = l2.as_mut().unwrap();
                        let next = n.next.take();
                        let n = l2;
                        l2 = next;
                        n
                    };
                    std::mem::replace(node, n);
                    let mut n = node.as_mut().unwrap();
                    node = &mut n.next;
                },
                (None, a) => {
                    std::mem::replace(node, l2);
                    break;
                },
                (a, None) => {
                    std::mem::replace(node, l1);
                    break;
                }
            }
        }
        ret
    }
}
*/
