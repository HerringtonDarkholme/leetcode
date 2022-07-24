use std::collections::VecDeque;
struct MyStack {
    pushed: VecDeque<i32>,
    to_pop: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Self {
            pushed: VecDeque::new(),
            to_pop: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.pushed.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if self.pushed.is_empty() {
            std::mem::swap(&mut self.pushed, &mut self.to_pop);
        }
        while self.pushed.len() > 1 {
            let ele = self.pushed.pop_front().unwrap();
            self.to_pop.push_back(ele);
        }
        self.pushed.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        let e = self.pop();
        self.push(e);
        e
    }

    fn empty(&self) -> bool {
        self.pushed.is_empty() && self.to_pop.is_empty()
    }
}


//
/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
