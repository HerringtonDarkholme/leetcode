use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        color_the_grid(m, n)
    }
}

const M: i64 = 1_000_000_007;
fn color_the_grid(m: i32, n: i32) -> i32 {
    let mut map = HashMap::new();
    let set = get_moves(m);
    for &j in set.iter() {
        map.insert(j, 1);
    }
    for i in 1..n {
        let mut dp = HashMap::new();
        for &a in set.iter() {
            for &b in set.iter() {
                if a & b == 0 {
                    let n = *dp.get(&a).unwrap_or(&0) + *map.get(&b).unwrap_or(&0);
                    dp.insert(a, n % M);
                }
            }
        }
        map = dp;
    }
    let mut r = 0;
    for (_, v) in map {
        r += v;
        r %= M;
    }
    r as i32
}

fn get_moves(m: i32) -> HashSet<u64> {
    let mut set = HashSet::new();
    if m == 0 {
        set.insert(0);
        return set
    }
    for j in get_moves(m-1) {
        for i in 0..3 {
            let n = 1 << i;
            if n & j == 0 {
                set.insert(j << 3 | n);
            }
        }
    }
    set
}
// 1,3 => 12
// 1,1 => 3
// 2,1 => 6
// 2,2 => 18
// 2,3 => 54
