use crate::util::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // no, reverse list then traverse is too ugly
        let mut a = &head;
        let mut v = Vec::new();
        while let Some(n) = a {
            v.push(n.val);
            a = &n.next;
        }
        if v.is_empty() {
            return true;
        }
        let mut i = 0;
        let mut j = v.len() - 1;
        while i < j {
            if v[i] != v[j] {
                return false
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (linked![], true),
        (linked![1], true),
        (linked![1, 1], true),
        (linked![1, 2, 1], true),
        (linked![1, 2, 2, 1], true),
        (linked![1, 2, 3, 1], false),
    ], Solution::is_palindrome);
}
