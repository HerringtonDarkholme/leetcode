use std::collections::VecDeque;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students: VecDeque<_> = students.into_iter().collect();
        let mut f = 0;
        let mut fail = 0;
        while let Some(s) = students.pop_front() {
            if s == sandwiches[f] {
                f += 1;
                fail = 0;
            } else {
                students.push_back(s);
                fail += 1;
            }
            if fail == students.len() {
                break;
            }
        }
        students.len() as i32
    }
}
