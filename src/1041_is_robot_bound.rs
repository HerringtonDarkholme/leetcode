#[derive(Eq, PartialEq)]
enum Direction {
    N, S, E, W,
}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        use Direction::*;
        let mut x = 0;
        let mut y = 0;
        let mut dir = N;
        for i in instructions.chars() {
            match i {
                'G' => match dir {
                    N => y += 1,
                    S => y -= 1,
                    E => x += 1,
                    W => x -= 1,
                },
                'L' => match dir {
                    N => dir = W,
                    S => dir = E,
                    E => dir = N,
                    W => dir = S,
                },
                'R' => match dir {
                    N => dir = E,
                    S => dir = W,
                    E => dir = S,
                    W => dir = N,
                },
                _ => (),
            }
        }
        dir != N || (x == 0 && y == 0)
    }
}
