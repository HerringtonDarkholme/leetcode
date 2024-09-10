impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        insert(head)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp
    }
    a
}

fn insert(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head?;
    if let Some(tail) = head.next.take() {
        let tv = tail.val;
        let mut new = ListNode::new(gcd(head.val, tv));
        new.next = insert(Some(tail));
        head.next = Some(Box::new(new));
    }
    Some(head)
}
