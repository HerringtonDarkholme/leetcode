impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut keys = vec![-1; 26];
        let mut start = 0;
        for c in key.bytes() {
            if c == b' ' {
                continue;
            }
            let i = (c - b'a') as usize;
            if keys[i] >=0 {
                continue;
            }
            keys[i] = start;
            start += 1;
        }
        let mut ret = String::new();
        for c in message.bytes() {
            if c == b' ' {
                ret.push(' ');
                continue;
            }
            let i = (c - b'a') as usize;
            let decoded = (keys[i] as u8 + b'a') as char;
            ret.push(decoded);
        }
        ret
    }
}

// impl Solution {
//     pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
//         let mut matrix = vec![vec![-1; n as usize]; m as usize];
//         let mut seen = vec![vec![false; n as usize]; m as usize];
//         let mut dr = vec![0, 1, 0, -1];
//         let mut dc = vec![1, 0 , -1, 0];
//         let mut x = 0;
//         let mut y = 0;
//         let mut di = 0;
//         while let Some(h) = head {
//             let val = h.val;
//             matrix[x as usize][y as usize] = val;
//             seen[x as usize][y as usize] = true;
//             let cr = x + dr[di];
//             let cc = y + dc[di];
//             if cr >= 0 && cr < m && cc >= 0 && cc < n && !seen[cr as usize][cc as usize] {
//                 x = cr;
//                 y = cc;
//             } else {
//                 di = (di + 1) % 4;
//                 x += dr[di];
//                 y += dc[di];
//             }
//             head = h.next;
//         }
//         matrix
//     }
// }
//     x--------------f
// [0, 0, 1, 0]
// use std::collections::VecDeque;
// const M: i64 = 1_000_000_007;
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut people = VecDeque::new();
        people.resize(forget as usize, 0);
        people[0] = 1;
        let mut spread = 0;
        let mut known = 1;
        people[0] = 1;
        for _ in 1..n {
            let f = people[forget as usize - 1];
            if known >= f {
                known -= f;
            } else {
                known = (known + M - f) % M;
            }
            if spread >= f {
                spread -= f;
            } else {
                spread = (spread + M - f) % M;
            }
            spread = (spread + people[delay as usize - 1]) % M;
            known = (known + spread) % M;
            people.pop_back();
            people.push_front(spread);
        }
        (known % M) as i32
    }
}

use std::collections::VecDeque;
const M: i64 = 1_000_000_007;
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut dp = vec![vec![0; c]; r];
        let mut ret = 0;
        for x in 0..r {
            for y in 0..c {
                ret += dfs(r, c, x as i32, y as i32, &mut dp, &grid);
                ret %= M;
            }
        }
        ret as i32
    }
}

fn dfs(r: usize, c: usize, row: i32, col: i32, dp: &mut Vec<Vec<i64>>,  grid: &Vec<Vec<i32>>) -> i64 {
    if dp[row as usize][col as usize] > 0 {
        return dp[row as usize][col as usize]
    }

    let mut count = 1;
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nr = row + dx;
        let nc = col + dy;
        if !(nr >= 0 && nr < r as i32 && nc >=0 && nc < c as i32) {
            continue;
        }
        let val = grid[row as usize][col as usize];
        let neighbor = grid[nr as usize][nc as usize];
        if val < neighbor {
            let neighbor_count = dfs(r, c, nr, nc, dp, grid);
            count = (count + neighbor_count) % M;
        }
    }
    dp[row as usize][col as usize] = count;
    count
}
