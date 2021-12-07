impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut r = 0;
        while let Some(h) = head {
            r = r * 2 + h.val;
            head = h.next;
        }
        r
    }
}
