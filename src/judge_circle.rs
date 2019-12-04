// leetcode 657
pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0i32;
        let mut y = 0i32;
        for m in moves.chars() {
            match m {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => (),
            }
        }
        x == 0 && y == 0
    }
}
