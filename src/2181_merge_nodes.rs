impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        to_list(merge(head))
    }
}
fn merge(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut sum = 0;
    while let Some(r) = head {
        if r.val == 0 && sum != 0 {
            ret.push(sum);
            sum = 0;
        } else {
            sum += r.val;
        }
        head = r.next;
    }
    ret
}
fn to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut ret = None;
    let mut node = &mut ret;
    for n in nums {
        let nd = ListNode::new(n);
        *node = Some(Box::new(nd));
        node = &mut node.as_mut().unwrap().next;
    }
    ret
}
