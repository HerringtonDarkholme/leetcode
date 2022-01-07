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
struct Solution {
    data: Vec<i32>,
}
use rand::{thread_rng, Rng};


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut data = vec![];
        while let Some(h) = head {
            data.push(h.val);
            head = h.next;
        }
        Self {
            data
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0, self.data.len());
        self.data[i]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
