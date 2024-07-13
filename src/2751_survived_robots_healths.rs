impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let mut bots = vec![];
        let dirs: Vec<_> = directions.chars().collect();
        for i in 0..positions.len() {
            bots.push((positions[i], healths[i], dirs[i], i));
        }
        bots.sort();
        let mut ret = vec![];
        let mut stack = vec![];
        for (pos, mut health, dir, i) in bots {
            if dir == 'R' {
                stack.push((i, health));
                continue;
            }
            while let Some((j, h)) = stack.pop() {
                if h > health {
                    health = 0;
                    stack.push((j, h - 1));
                    break;
                } else if h == health {
                    health = 0;
                    break;
                } else {
                    health -= 1;
                }
            }
            if health > 0 {
                ret.push((i, health));
            }
        }
        ret.extend(stack);
        ret.sort();
        ret.into_iter().map(|n| n.1).collect()
    }
}
