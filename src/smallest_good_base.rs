pub struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: i64 = n.parse().unwrap();
        let upper = find_max(n);
        let mut base = n - 1;
        for i in 3..=upper {
            let b = compute(n, i);
            if b > 0 {
                base = b;
            }
        }
        format!("{}", base)
    }
}

fn find_max(mut n: i64) -> i64 {
    let mut r = 0;
    while n > 0 {
        r += 1;
        n = n >> 1;
    }
    r
}

fn compute(n: i64, i: i64) -> i64 {
    let mut low = 2;
    let mut high = n;
    //let mut high = 10i64.pow(((n as f64).log10() / (i as f64)) as u32) + 20;
    while low < high {
        let mid = low + (high - low) / 2;
        let r = aux(i, mid);
        if r >= n {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let r = aux(i, low);

    if r == n { low } else { -1 }
}

fn aux(i: i64, b: i64) -> i64 {
    let mut r = 0;
    let mut a = 1;
    for c in 0..i {
        if r > i64::max_value() - a {
            return i64::max_value()
        }
        r += a;
        if a > i64::max_value() / b && c != i - 1 {
            return i64::max_value()
        }
        a *= b;
    }
    r
}
