impl Solution {
    pub fn tribonacci(mut n: i32) -> i32 {
        let (mut a, mut b, mut c) = (0, 1, 1);
        while n > 2 {
            (a, b, c) = (b, c, a + b + c);
            n -= 1;
        }
        if n == 0 { a } else if n == 1 { b } else { c }
    }
}
