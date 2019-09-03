struct MyQueue {
    stack: Vec<i32>,
    rev_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: vec![],
            rev_stack: vec![],
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        while !self.rev_stack.is_empty() {
            self.stack.push(self.rev_stack.pop().unwrap());
        }
        self.stack.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.peek();
        self.rev_stack.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        while !self.stack.is_empty() {
            self.rev_stack.push(self.stack.pop().unwrap());
        }
        *self.rev_stack.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack.is_empty() && self.rev_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
