// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut n = &head;
        while let Some(inner) = n.as_ref() {
            v.push(inner.val);
            n = &inner.next;
        }
        let mut sum = 0;
        let mut map = HashMap::new();
        let mut ret = vec![];
        let mut index = 0;
        for i in 0..v.len() {
            sum += v[i];
            if sum == 0 {
                index = 0;
                map = HashMap::new();
            } else if map.contains_key(&sum) {
                let mut j = index - 1;
                index = map[&sum];
                sum -= v[i];
                while j >= index {
                    map.remove(&sum);
                    sum -= ret[j];
                    j -= 1;
                }
            } else {
                map.insert(sum, index + 1);
                if index >= ret.len() {
                    ret.push(v[i])
                } else {
                    ret[index] = v[i];
                }
                index += 1;
            }
        }
        while ret.len() > index {
            ret.pop();
        }
        let mut h = None;
        let mut n = &mut h;
        for num in ret {
            n.replace(Box::new(ListNode{val: num, next: None}));
            n = &mut n.as_mut().unwrap().next;
        }
        h
    }
}
