use std::collections::HashSet;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let origin = stones.len();
        let mut unions: Vec<(HashSet<i32>, HashSet<i32>)> = vec![];
        for stone in stones {
            let mut u1 = None;
            let mut u2 = None;
            let (r, c) = (stone[0], stone[1]);
            let mut next = vec![];
            for mut u in unions {
                if !u.0.contains(&r) && !u.1.contains(&c) {
                    next.push(u);
                    continue;
                }
                if u1.is_none() {
                    u1 = Some(u);
                } else {
                    u2 = Some(u);
                }
            }
            let mut u = match (u1, u2) {
                (Some(mut u1), Some(u2)) => {
                    u1.0.extend(u2.0);
                    u1.1.extend(u2.1);
                    u1
                },
                (Some(mut u1), _) => u1,
                _ => (HashSet::new(), HashSet::new())
            };
            u.0.insert(r);
            u.1.insert(c);
            next.push(u);
            unions = next;
        }
        origin as i32 - unions.len() as i32
    }
}
