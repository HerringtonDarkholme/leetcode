use std::collections::BinaryHeap;
struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<i32>,
    even: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
            even: true,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.even {
            self.large.push(num);
            self.small.push(-self.large.pop().unwrap());
        } else {
            self.small.push(-num);
            self.large.push(-self.small.pop().unwrap());
        }
        self.even = !self.even
    }

    fn find_median(&self) -> f64 {
        if self.even {
            (*self.large.peek().unwrap() - *self.small.peek().unwrap()) as f64 / 2.0
        } else {
            -*self.small.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
