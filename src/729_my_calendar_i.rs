struct MyCalendar {
    root: IntervalTree
}

trait IntervalTreeOp {
    fn insert(&mut self, start: i32, end: i32) -> bool;
}

type IntervalTree = Option<Box<IntervalTreeNode>>;

struct IntervalTreeNode {
    start: i32,
    end: i32,
    left: IntervalTree,
    right: IntervalTree,
}

impl IntervalTreeOp for IntervalTree {
    fn insert(&mut self, start: i32, end: i32) -> bool {
        if self.is_none() {
            self.replace(Box::new(IntervalTreeNode{
                start,
                end,
                left: None,
                right: None,
            }));
            return true
        }
        let mut n = self.as_mut().unwrap();
        if end <= n.start {
            n.left.insert(start, end)
        } else if start >= n.end {
            n.right.insert(start, end)
        } else {
            false
        }
    }
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            root: None,
        }
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        self.root.insert(start, end)
    }
}

/*
use std::collections::BTreeSet;


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
struct MyCalendar {
    start: BTreeSet<i32>,
    end: BTreeSet<i32>,
}

impl MyCalendar {

    fn new() -> Self {
        Self {
            start: BTreeSet::new(),
            end: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let nearest_start = self.start.range(..end).rev().next();
        let nearest_end = self.end.range(start+1..).next();
        match (nearest_start, nearest_end) {
            (Some(&n_start), Some(&n_end)) => {
                if n_start >= start || n_end <= end {
                    return false
                }
                let n_end = *self.end.range(n_start+1..).next().unwrap();
                if n_end > start {
                    return false
                }
            }
            _ => (),
        }
        self.start.insert(start);
        self.end.insert(end);
        true
    }
}
*/

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
