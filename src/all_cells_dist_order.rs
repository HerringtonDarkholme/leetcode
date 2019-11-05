pub struct Solution;

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]; r*c];
        // because we know max distance, we can use
        // counting sort, count how many elemnts in one bucket
        let mut counter = vec![0; r + c + 1];
        for x in 0..r {
            for y in 0..c {
                let dist = (x - r0).abs() + (y - c0).abs();
                counter[dist + 1] += 1;
            }
        }
        for i in 1..(r+c+1) {
            // count offset
            counter[i] += counter[i - 1];
        }
        for x in 0..r {
            for y in 0..c {
                let dist = (x - r0).abs() + (y - c0).abs();
                ret[counter[dist]] = vec![x, y];
                counter[dist] += 1;
            }
        }
        ret
    }
}
