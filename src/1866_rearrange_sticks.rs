const M: i64 = 1_000_000_007;

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut cache = vec![vec![-1; k + 1]; n + 1];
        aux(n, k, &mut cache) as i32
    }
}

fn aux(n: usize, k: usize, cache: &mut Vec<Vec<i64>>) -> i64 {
    if k == n {
        return 1
    }
    if k == 0 {
        return 0
    }
    if cache[n][k] != -1 {
        return cache[n][k]
    }
    cache[n][k] = (aux(n - 1, k - 1, cache) + aux(n - 1, k, cache) * (n as i64 - 1)) % M;
    cache[n][k]
}
