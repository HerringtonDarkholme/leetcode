impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![-1; mat[0].len()]; mat.len()];
        let mut frontier = vec![];
        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 0 {
                    frontier.push(r * mat[0].len() + c);
                    result[r][c] = 0;
                }
            }
        }
        let mut dist = 1;
        while !frontier.is_empty() {
            let mut next = vec![];
            for dim in frontier {
                let r = (dim / mat[0].len()) as i32;
                let c = (dim % mat[0].len()) as i32;
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nr = dx + r;
                    let nc = dy + c;
                    if nr < 0 || nr >= mat.len() as i32 || nc < 0 || nc >= mat[0].len() as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if result[nr][nc] >= 0 {
                        continue;
                    }
                    result[nr][nc] = dist;
                    next.push(nr * mat[0].len() + nc);
                }
            }
            dist += 1;
            frontier = next;
        }
        result
    }
}
