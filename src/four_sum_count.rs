struct Solution;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut h = std::collections::HashMap::new();
        let mut ret = 0;
        for i in 0..a.len() {
            let ai = a[i];
            for j in 0..b.len() {
                let bj = b[j];
                *h.entry(ai+bj).or_insert(0) += 1;
            }
        }
        for i in 0..c.len() {
            let ci = c[i];
            for j in 0..d.len() {
                let dj = d[j];
                if let Some(c1) = h.get(&-(ci+dj)) {
                    ret += c1;
                }
            }
        }
        ret
    }
}
