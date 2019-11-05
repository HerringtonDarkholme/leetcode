pub struct Solution;

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for x in 0..r {
            for y in 0..c {
                ret.push(vec![x, y]);
            }
        }
        ret.sort_by_key(|v| (v[0] - r0).abs() + (v[1] -c0).abs());
        ret
    }
}
