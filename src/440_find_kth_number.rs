impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut curr = 1i64;
        let mut k = k as i64 - 1;
        while k > 0 {
            let step = get_steps(n as i64, curr, curr + 1);
            if step <= k {
                curr += 1;
                k -= step;
            } else {
                curr *= 10;
                k -= 1;
            }
        }
        curr as i32
    }
}

fn get_steps(n: i64, mut prefix1: i64, mut prefix2: i64) -> i64 {
    let mut steps = 0;
    while prefix1 <= n {
        steps += prefix2.min(n + 1) - prefix1;
        prefix1 *= 10;
        prefix2 *= 10;
    }
    steps
}
