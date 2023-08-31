use std::collections::HashMap;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let mut existing = HashMap::new();
        let mut day = 0;
        for task in tasks {
            day += 1;
            if let Some(&d) = existing.get(&task) {
                day = day.max(d + 1); // wait or already past
            }
            existing.insert(task, day + space as i64);
        }
        day
    }
}

// {1: 8, 2: 9,3: 10} d=2
// d=7
