impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut curr = vec![0];
        while let Some(c) = curr.pop() {
            let c = c as usize;
            if visited[c] {
                continue;
            }
            visited[c] = true;
            curr.extend(rooms[c].iter().copied());
        }
        visited.iter().all(|b| *b)
    }
}
