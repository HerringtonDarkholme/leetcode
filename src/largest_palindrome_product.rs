pub struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9
        }
        let max = 10i64.pow(n as u32) - 1;
        let min = 10i64.pow(n as u32 - 1);
        let mut half = max - 1;
        loop {
            let pal = Solution::gen_palindrom(half);
            let mut n = max;
            while n > min {
                let i = pal / n;
                if i > n {
                    break;
                }
                if pal % n == 0 && pal / n < max {
                    return ((n % 1337) * (pal / n % 1337)) as i32 % 1337
                }
                n -= 2;
            }
            half -= 1;
        }
        0
    }

    fn gen_palindrom(mut half: i64) -> i64 {
        let mut first = half;
        let mut second = 0;
        while half != 0 {
            second = second * 10 + half % 10;
            half /= 10;
            first *= 10;
        }
        first + second
    }
}
