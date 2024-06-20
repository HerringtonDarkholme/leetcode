impl Solution {
    pub fn max_distance(mut pos: Vec<i32>, m: i32) -> i32 {
        pos.sort_unstable();
        let mut left = 1;
        let mut right = (pos[pos.len() - 1] - pos[0]) / (m - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if can_place(mid, &pos, m) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if can_place(left, &pos, m) { left } else { left - 1 }
    }
}
fn can_place(gap: i32, pos: &[i32], m: i32) -> bool {
    let mut prev = pos[0];
    let mut balls = 1;
    for i in 1..pos.len() {
        let curr = pos[i];
        if curr - prev >= gap {
            balls += 1;
            prev = curr;
        }
    }
    balls >= m
}
