impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let top = tops[0];
        let bottom = bottoms[0];
        let top_rotation = find(top, &tops, &bottoms);
        let bottom_rotation = find(bottom, &bottoms, &tops);
        if top_rotation == -1 {
            bottom_rotation
        } else if bottom_rotation == -1 {
            top_rotation
        } else {
            top_rotation.min(bottom_rotation)
        }
    }
}

fn find(n: i32, tops: &[i32], bottoms: &[i32]) -> i32 {
    let mut top_mut = 0;
    let mut bot_mut = 0;
    for i in 1..tops.len() {
        if tops[i] == n {
            bot_mut += if bottoms[i] == n {
                0
            } else {
                1
            };
        } else if bottoms[i] == n {
            top_mut += 1;
        } else {
            return -1;
        }
    }
    top_mut.min(bot_mut + 1)
}
