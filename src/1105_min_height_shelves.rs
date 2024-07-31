use std::collections::HashMap;
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf: i32) -> i32 {
        let mut dp = HashMap::new();
        dp.insert((shelf, 0), 0);
        for book in books {
            let (w, h) = (book[0], book[1]);
            let mut next = HashMap::new();
            for ((width, max), height) in dp {
                if width >= w {
                    let new_max = max.max(h);
                    let new_h = height + new_max - max;
                    if let Some(old_height) = next.get_mut(&(width - w, new_max)) {
                        *old_height = new_h.min(*old_height);
                    } else {
                        next.insert((width - w, new_max), new_h);
                    }
                }
                let new_h = height + h;
                if let Some(old_height) = next.get_mut(&(shelf - w, h)) {
                    *old_height = new_h.min(*old_height);
                } else {
                    next.insert((shelf - w, h), new_h);
                }
            }
            dp = next;
        }
        dp.into_values().min().unwrap()
    }
}

// (4,2), (3, 0)
// (5,3), (4,1), (4,2)
