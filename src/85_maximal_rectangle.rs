impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut vertical = vec![0i32; matrix[0].len() + 1];
        let mut max = 0;
        for mut row in matrix {
            let mut horizontal: Vec<usize> = vec![];
            row.push('0');
            for c in 0..row.len() {
                let cell = (row[c] as u8 - b'0') as i32;
                vertical[c] = vertical[c] * cell + cell;
                let v = vertical[c];
                while !horizontal.is_empty() && vertical[horizontal[horizontal.len() - 1]] >= v {
                    let h = vertical[horizontal.pop().unwrap()];
                    let w = horizontal.last().map(|&s| c - s - 1).unwrap_or(c) as i32;
                    max = max.max(w * h);
                }
                horizontal.push(c);
            }
        }
        max
    }
}
