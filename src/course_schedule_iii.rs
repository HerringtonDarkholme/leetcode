pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|c| c[1]);
        let mut queue = BinaryHeap::new();
        let mut start = 0;
        for course in courses {
            let (t, end) = (course[0], course[1]);
            if start + t <= end {
                queue.push(t);
                start += t;
            } else if !queue.is_empty() && *queue.peek().unwrap() > t {
                start += t - queue.pop().unwrap();
                queue.push(t);
            }
        }
        queue.len() as i32
    }
}

/* TLE !!
use std::collections::HashMap;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|c| c[1]);
        let mut cache = vec![HashMap::new(); courses.len()];
        recurse(&courses, 0, 0, &mut cache)

    }
}

fn recurse(courses: &Vec<Vec<i32>>, start: usize, time: i32, cache: &mut Vec<HashMap<i32, i32>>) -> i32 {
    if start == courses.len() {
        return 0
    }
    if let Some(t) = cache[start].get(&time) {
        return *t
    }
    let course = &courses[start];
    let t = if course[0] + time > course[1] {
        recurse(courses, start + 1, time, cache)
    } else {
        let take = 1 + recurse(courses, start + 1, time + course[0], cache);
        let no_take = recurse(courses, start + 1, time, cache);
        take.max(no_take)
    };
    cache[start].insert(time, t);
    t
}
*/
