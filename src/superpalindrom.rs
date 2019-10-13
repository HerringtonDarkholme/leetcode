pub struct Solution;
impl Solution {
    pub fn superpalindromes_in_range(l: String, r: String) -> i32 {
        let l: i64 = l.parse().unwrap();
        let r: i64 = r.parse().unwrap();
        let mut ret = 0;
        for i in 0..=100000 {
            let j = gen(i);
            let j = j * j;
            if j > r {
                break;
            }
            if j >= l && j == reverse(j) {
                ret += 1;
            }
        }
        for i in 0..=100000 {
            let j = gen2(i);
            let j = j * j;
            if j > r {
                break;
            }
            if j >= l && j == reverse(j) {
                ret += 1;
            }
        }
        ret
    }
}

fn gen(i: i64) -> i64 {
    let a = format!("{}", i);
    format!("{}{}", i, a.chars().rev().collect::<String>()).parse().unwrap()
}
fn gen2(i: i64) -> i64 {
    if i < 10 {
        return i
    }
    let a = format!("{}", i);
    format!("{}{}", i/10, a.chars().rev().collect::<String>()).parse().unwrap()
}
fn reverse(mut n: i64) -> i64 {
    let mut r = 0;
    while n > 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}
