use std::collections::HashSet;
impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut known: Vec<_> = (0..n).collect();
        known[first_person as usize] = 0;
        meetings.sort_by_key(|n| n[2]);
        let mut temp = known.clone();
        let mut seen = HashSet::new();
        let mut last_time = 0;
        for meeting in meetings {
            if meeting[2] != last_time {
                for key in seen {
                    let root = find(&temp, key);
                    if root == 0 { known[key as usize] = 0; }
                }
                temp = known.clone();
                seen = HashSet::new();
                last_time = meeting[2];
            }
            seen.insert(meeting[0]);
            seen.insert(meeting[1]);
            union(&mut temp, meeting[0], meeting[1]);
        }
        for key in seen {
            let root = find(&temp, key);
            if root == 0 { known[key as usize] = 0; }
        }
        known.into_iter().enumerate().filter_map(|n| if n.1 == 0 {
            Some(n.0 as i32)
        } else { None }).collect()
    }
}
fn union(parents: &mut Vec<i32>, x: i32, y: i32) {
    let x = find(parents, x);
    let y = find(parents, y);
    let (x, y) = if x < y { (x, y) } else { (y, x) };
    parents[y as usize] = x;
}
fn find(parents: &Vec<i32>, mut x: i32) -> i32 {
    while parents[x as usize] != x {
        x = parents[x as usize];
    }
    x
}
