impl Solution {
    pub fn minimum_perimeter(mut needed_apples: i64) -> i64 {
        let mut r = 0;
        let mut i = 1;
        loop {
            needed_apples -= compute_r(r);
            if needed_apples <= 0 {
                break r * 8
            }
            r += 1;
            i += 1;
        }
    }
}

fn compute_r(r: i64) -> i64 {
    let half_edge = (r + (2 * r - 1)) * r / 2;
    let whole_edge = half_edge * 2 - r; // dup mid
    whole_edge * 4 + r * 2* 4
}
