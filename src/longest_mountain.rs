#[derive(Eq, PartialEq)]
enum Mountain {
    Initial,
    Ascend,
    Descend,
}

use self::Mountain::{Initial, Ascend, Descend};

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut state = Initial;
        let mut max = 0;
        let mut left = 0;
        for i in 1..a.len() {
            let last = a[i - 1];
            let curr = a[i];
            if last > curr {
                match state {
                    Ascend => {
                        state = Descend;
                        max = max.max(i + 1 - left);
                    },
                    Descend => {
                        max = max.max(i + 1 - left);
                    },
                    Initial => {
                        left = i;
                    },
                }
            } else if last < curr {
                match state {
                    Ascend => {},
                    Descend => {
                        state = Ascend;
                        left = i - 1;
                    },
                    Initial => {
                        state = Ascend;
                        left = i - 1;
                    },
                }
            } else {
                left = i;
                state = Initial;
            }
        }
        max as i32
    }
}

/*
impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut state = Initial;
        let mut max = 0;
        let mut left = 0;
        for i in 1..a.len() {
            let last = a[i - 1];
            let curr = a[i];
            if last > curr && state == Ascend {
                state = Descend;
            } else if last < curr && state != Ascend {
                if state == Descend {
                    max = max.max(i - left);
                }
                state = Ascend;
                left = i - 1;
            } else if last == curr {
                if state == Descend {
                    max = max.max(i - left);
                }
                left = i;
                state = Initial;
            }
        }
        if state == Descend {
            max = max.max(a.len() - left);
        }
        max as i32
    }
}
*/
