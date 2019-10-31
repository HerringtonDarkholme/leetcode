pub struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        search(sx, sy, tx, ty)
    }
}

/*
fn search(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    loop {
        if sx == tx && sy == ty {
            break true
        }
        if ty > tx {
            if ty - tx < sy {
                break false
            }
            ty = (ty - sy) % tx + sy;
        } else {
            if tx - ty < sx {
                break false
            }
            tx = (tx - sx) % ty + sx;
        }
    }
}
*/

fn search(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while sx < tx && sy < ty {
        if tx > ty { tx %= ty; }
        else { ty %= tx; }
    }
    sx == tx && sy <= ty && (ty - sy) % sx == 0 ||
    sy == ty && sx <= tx && (tx - sx) % sy == 0

}
