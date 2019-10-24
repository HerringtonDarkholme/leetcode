pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let n = n as i64;
        let mut low = 0;
        let mut high = 2 * 1_000_000_000;
        let ab = a * b / gcd(a, b);
        let ac = a * c / gcd(a, c);
        let bc = b * c / gcd(b, c);
        let abc = a * bc / gcd(a, bc);
        while low < high {
            let m = low + (high - low) / 2;
            let cnt = m/a + m/b + m/c - m/ab - m/ac - m/bc + m/abc;
            if cnt < n {
                low = m + 1;
            } else {
                high = m;
            }
        }
        low as i32
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a % b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    b
}
