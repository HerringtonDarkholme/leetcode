impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut r = 0;
        while n / 5 > 0 {
            r += n / 5;
            n /= 5;
        }
        r
    }
}
