pub struct Solution;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut row = row;
        let len = row.len();
        let mut idx = vec![0; len];
        for (k, i) in row.iter().enumerate() {
            idx[*i as usize] = k;
        }
        let mut ret = 0;
        for i in 0..(len / 2) {
            if row[2*i] / 2 == row[2*i+1] / 2 {
                continue;
            }
            let p = row[2 * i];
            let q = if p % 2 == 1 { p  - 1 } else { p + 1 };
            let idx_q = idx[q as usize];
            row.swap(2*i + 1, idx_q);
            idx[row[idx_q] as usize] = idx_q;
            ret += 1;
        }
        ret
    }
}
