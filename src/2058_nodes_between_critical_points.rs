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
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut prev_direction = 0; // 1 incr, -1 decr, 0 even
        let mut last_critical = -1;
        let mut init_critical = -1;
        let mut min_dist = -1;
        let mut max_dist = -1;
        let h = head.unwrap();
        let mut last_val = h.val;
        let mut head = h.next;
        let mut index = 1;
        while let Some(h) = head {
            let direction = if h.val == last_val { 
                0 
            } else if h.val > last_val {
                1
            } else {
                -1
            };
            if direction != 0 && prev_direction == -direction {
                if last_critical >= 0 {
                    min_dist = if min_dist > 0 {
                        min_dist.min(index - last_critical)
                    } else {
                        index - last_critical
                    };
                }
                if init_critical < 0 {
                    init_critical = index;
                } else {
                    max_dist = index - init_critical;
                }
                last_critical = index;
            }
            last_val = h.val;
            prev_direction = direction;
            head = h.next;
            index += 1;
        }
        vec![min_dist, max_dist]
    }
}
