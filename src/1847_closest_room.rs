impl Solution {
    pub fn closest_room(mut rooms: Vec<Vec<i32>>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut queries = queries
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        queries.sort_by_key(|v| -v.1[1]);
        rooms.sort_by_key(|v| -v[1]);
        let mut available = std::collections::BTreeSet::new();
        let mut ans = vec![-1; queries.len()];
        for (i, query) in queries {
            while !rooms.is_empty() && rooms[0][1] >= query[1] {
                let room = rooms.remove(0);
                available.insert(room[0]);
            }
            let ceil = available.range(query[0]..).next();
            let floor = available.range(..query[0]).rev().next();
            let a = match (floor, ceil) {
                (Some(i), None) => i,
                (None, Some(i)) => i,
                (Some(f), Some(c)) => {
                    if query[0] - f > c - query[0] {
                       c 
                    } else {
                        f
                    }
                }
                _ => &-1,
            };
            ans[i] = *a;
        }
        ans
    }
}
