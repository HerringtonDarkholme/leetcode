impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut bytes = expression.as_bytes();
        let (mut sign, mut i) = if bytes[0] == b'+' {
            (1, 1)
        } else if bytes[0] == b'-' {
            (-1, 1)
        } else {
            (1, 0)
        };
        let mut numerator = 0;
        let mut denominator = 1;
        while i < bytes.len() {
            let (mut n, mut d) = (0, 0);
            while bytes[i] != b'/' {
                n = n * 10 + (bytes[i] - b'0') as i64;
                i += 1;
            }
            i += 1;
            while i < bytes.len() && bytes[i] != b'+' && bytes[i] != b'-' {
                d = d * 10 + (bytes[i] - b'0') as i64;
                i += 1;
            }
            let new = merge(numerator, denominator, n, d, sign);
            numerator = new.0;
            denominator = new.1;
            if i < bytes.len() {
                sign = if bytes[i] == b'+' { 1 } else { -1 };
                i += 1;
            }
        }
        format!("{numerator}/{denominator}")
    }
}
fn merge(n1: i64, d1: i64, n2: i64, d2: i64, sign: i64) -> (i64, i64) {
    let d = gcd(d1, d2);
    let n = n1 * d2 / d + sign * n2 * d1 / d;
    let d = d1 * d2 / d;
    let g = gcd(n, d);
    (n/g, d/g)
}
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let (a, b) = (a.abs(), b.abs());
    let (mut a, mut b) = if a < b { (a, b) } else { (b, a) };
    while a > 0 {
        let temp = a;
        a = b % a;
        b = temp;
    }
    b
}
