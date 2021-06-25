const M: u64 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        find_paths(
            m as usize, 
            n as usize, 
            max_move as usize, 
            start_row as usize,
            start_column as usize
        ) as i32
    }
}

fn find_paths(m: usize, n: usize, steps: usize, row: usize, col: usize) -> u64 {
    let mut pos = vec![vec![0; n]; m];
    pos[row][col] = 1;
    let mut s = 0;
    for i in 0..steps {
        s += count(&pos, m, n) % M;
        s %= M;
        pos = next_pos(pos, m, n);
    }
    s
}

fn next_pos(pos: Vec<Vec<u64>>, m: usize, n: usize) -> Vec<Vec<u64>> {
    let mut next = vec![vec![0; n]; m];
    let m = m as i32;
    let n = n as i32;
    for x in 0..m {
        for y in 0..n {
            for (dx, dy) in &[(0,1), (0,-1),(1, 0), (-1,0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= m || ny < 0 || ny >= n {
                    continue;
                }
                next[x as usize][y as usize] += pos[nx as usize][ny as usize];
                next[x as usize][y as usize] %= M;
            }
        }
    }
    next
}

fn count(pos: &Vec<Vec<u64>>, m: usize, n: usize) -> u64 {
    let up = pos[0].iter().sum::<u64>() % M;
    let down = pos[m-1].iter().sum::<u64>() % M;
    let lateral = pos.iter().map(|p| {
        (p[0] + p[n-1]) % M
    }).sum::<u64>() % M;
    (up + down + lateral) % M
}
