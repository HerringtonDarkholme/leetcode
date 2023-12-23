impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut point = (0, 0);
        let mut visited = std::collections::HashSet::new();
        visited.insert(point);
        for c in path.chars() {
            match c {
                'N' => point.1 += 1,
                'S' => point.1 -= 1,
                'E' => point.0 += 1,
                'W' => point.0 -= 1,
                _ => (),
            }
            if visited.contains(&point) {
                return true;
            }
            visited.insert(point);
        }
        false
    }
}
