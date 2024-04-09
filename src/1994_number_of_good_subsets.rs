const single: &[i32] = &[2,3,5,7,11,13,17,19,23,29];
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 1024];
        let nums = build_nums(nums);
        const M: i64 = 1_000_000_007;
        let mut one = 0;
        for (n, &count) in nums.iter().enumerate().skip(1) {
            if count == 0 { continue; }
            let n = n + 1;
            let mut next = counter.clone();
            let bit = to_bit(n as i32);
            if bit < 0 { continue; }
            let bit = bit as usize;
            next[bit] += count;
            next[bit] %= M;
            for j in 1..1024 {
                if j & bit != 0 { continue; }
                let nj = j | bit;
                next[nj] += (count * counter[j]) % M;
                next[nj] %= M;
            }
            counter = next;
        }
        let mut sum = 0;
        for c in counter { sum += c; sum %= M; }
        for _ in 0..nums[0] { sum *= 2; sum %= M; }
        sum as i32
    }
}
fn build_nums(v: Vec<i32>) -> Vec<i64> {
    let mut ret = vec![0; 30];
    for n in v { ret[n as usize - 1] += 1; }
    return ret;
}
fn to_bit(n: i32) -> i32 {
    let mut ret = 0;
    for i in 0..single.len() {
        let s = single[i];
        if n % s != 0 { continue; }
        if (n/s) % s == 0 { return -1; }
        ret |= 1 << i;
    }
    ret
}
