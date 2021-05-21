const M: i64 = 1_000_000_007;

impl Solution {
    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        let mut stack: Vec<(i64, i64)> = Vec::with_capacity(arr.len());
        let mut s = 0;
        let mut acc = 0;
        for i in arr {
            let i = i as i64;
            let mut c = 1;
            while stack.len() > 0 && i <= stack[stack.len() - 1].0 {
                let a = stack.pop().unwrap();
                acc -= a.0 * a.1;
                c += a.1;
            }
            acc += i * c;
            stack.push((i, c));
            s += acc % M;
        }
        (s % M) as i32
    }
}
