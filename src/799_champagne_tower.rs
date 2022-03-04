impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut rets = vec![poured as f64];
        for _ in 0..query_row {
            let mut next = vec![0.0; rets.len() + 1];
            for i in 0..rets.len() {
                let pour = rets[i];
                if pour <= 1.0 {
                    continue;
                }
                next[i] += (pour - 1.0) / 2.0;
                next[i + 1] += (pour - 1.0) / 2.0;
            }
            rets = next;
        }
        rets[query_glass as usize].min(1.0)
    }
}

/// given poured(i, j) is the water poured into glass at i, j
/// for next row glasses (i + 1, j) and (i+1, j+1) got
/// (p-1)/2 waterif p > 1.0, else next row glasses got 0.
/// So next row glass poured(i+1, j) = (poured(i, j-1) + poured(i, j)) / 2 - 1
