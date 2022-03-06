impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![vec![0; n + 1]; n + 1];
        count(n as i64, n as i64, &mut cache) as i32
    }
}
const M: i64 = 1_000_000_007;
fn count(p: i64, d: i64, cache: &mut Vec<Vec<i64>>) -> i64 {
    if p == 0 && d == 0 {
        return 1;
    }
    if p < 0 || d < 0 || d < p {
        return 0;
    }
    if cache[p as usize][d as usize] > 0 {
        return cache[p as usize][d as usize];
    }
    let mut ret = 0;
    ret += p * count(p - 1, d, cache);
    ret %= M;
    ret += (d - p) * count(p, d - 1, cache);
    ret %= M;
    cache[p as usize][d as usize] = ret;
    ret
}
//  Cnt(n P, m D)
//  n * Cnt(n-1, D+1)
//  m * Cnt(n, D-1)
