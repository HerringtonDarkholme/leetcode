
struct MyCalendarTwo {
    once: Vec<(i32, i32)>,
    twice: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        Self {
            once: vec![],
            twice: vec![],
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        for (s, e) in &self.twice {
            if start >= *e || end <= *s {
                continue;
            }
            return false;
        }
        for (s, e) in & self.once {
            if start >= *e || end <= *s {
                continue;
            }      
            let new_start = start.max(*s);
            let new_end = end.min(*e);
            self.twice.push((new_start, new_end));
        }
        self.once.push((start, end));
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
