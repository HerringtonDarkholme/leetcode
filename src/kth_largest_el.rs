use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let k = k as usize;
        let mut s = Self {
            k, heap,
        };
        for i in nums {
            s.push(i);
        }
        s
    }
    fn push(&mut self, val: i32) {
        let mut heap = &mut self.heap;
        if heap.len() >= self.k && *heap.peek().unwrap() > -val {
            heap.pop();
            heap.push(-val);
        } else if heap.len() < self.k {
            heap.push(-val);
        }
    }
    fn add(&mut self, val: i32) -> i32 {
        self.push(val);
        -*self.heap.peek().unwrap()
    }
}
