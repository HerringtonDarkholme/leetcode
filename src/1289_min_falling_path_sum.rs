impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (mut min1, mut min2)= ((0, 0), (0, 0));
        for row in grid {
            let (mut next1, mut next2)= ((i32::MAX, 0), (i32::MAX, 0));
            for (i, cell) in row.into_iter().enumerate() {
                let next = if i != min1.1 {
                    (min1.0 + cell, i)
                } else {
                    (min2.0 + cell, i)
                };
                if next < next1 {
                    next2 = next1;
                    next1 = next;
                } else if next < next2 {
                    next2 = next;
                }
            }
            (min1, min2) = (next1, next2);
        }
        min1.0
    }
}
