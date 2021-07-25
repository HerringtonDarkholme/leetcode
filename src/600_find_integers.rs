use std::collections::HashMap;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut cache = HashMap::new();
        find_int(n, &mut cache)
    }
}

fn find_int(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if n < 3 {
        return n + 1
    }
    if cache.contains_key(&n) {
        return cache[&n]
    }
    let i = find_nearest_2_power(n);
    let m = 2i32.pow(i as u32) - 1;
    let next2 = 2i32.pow(i as u32 - 1) - 1;
    let r = find_int(m, cache) + find_int(next2.min(n - m - 1), cache);
    cache.insert(n, r);
    r
}

fn find_nearest_2_power(n: i32) -> i32 {
    let mut i = 0;
    while 2i32.pow(i) <= n {
        i += 1;
    }
    i -= 1;
    i as i32
}




// f(n) = f(nearest-2-power) + f(min(remain, next-2-power))
