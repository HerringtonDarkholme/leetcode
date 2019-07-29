pub struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut v = vec![];
        let mut n = n;
        while n != 0 {
            if !v.is_empty() && *v.last().unwrap() < n % 10 {
                v = v.iter().map(|_| 9).collect();
                v.push(n % 10 - 1);
            } else {
                v.push(n % 10);
            }
            n = n / 10;
        }
        let mut ret = 0;
        while !v.is_empty() {
            ret = ret * 10 + v.pop().unwrap();
        }
        ret
    }
}
