pub struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let r = nth_magical_number(n as i64, a as i64, b as i64);
        (r % 1_000_000_007) as i32
    }
}

fn nth_magical_number(n: i64, a: i64, b: i64) -> i64 {
    let (a, b) = if a < b {
        (a, b)
    } else {
        (b, a)
    };
    if b % a == 0 {
        return n * a
    }
    let gcd = find_gcd(a, b);
    let lcm = a * (b / gcd);
    let mut low = n;
    let mut high = (a / gcd) * n;
    while low < high {
        let mid = low + (high - low) / 2;
        let num_a = mid * gcd + a - (mid * gcd) % a;
        let num_b = mid * gcd + b - (mid * gcd) % b;
        let count = count_order(num_a, a, b, lcm);
        let count_b = count_order(num_b, a, b, lcm);
        if count == n {
            return num_a
        }
        if count_b == n {
            return num_b
        }
        if count < n {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    -1
}

fn find_gcd(mut a: i64, mut b: i64) -> i64 {
    while b % a != 0 {
        let temp = b % a;
        b = a;
        a = temp;
    }
    a
}

fn count_order(num: i64, a: i64, b: i64, lcm: i64) -> i64 {
    num / a + num / b - num / lcm
}
