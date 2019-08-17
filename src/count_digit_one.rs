pub struct Solution;
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n = n as i64;
        let mut r = 0i64;
        let mut i = 1; // the place we count 1
        // compute 1's counton ones place, tens place, hundreds place....
        while n / i > 0 {
            // we always have ones before current place, say
            // 2331, current place is ones, we have
            // 233 ones on one place,
            // 230 ones on ten places
            // 200 ones on hundren places
            r += n / i / 10 * i;
            let c = n / i % 10;
            // if current place is one, add remaning one after current place
            // e.g. 214, we have 20 ones on ten places, counted above,
            // and 5 ones for 210, 211, 212, 213, 214
            if c == 1 {
                r += n % i + 1;
            } else if c > 1 {
                // current place larger than one, just add ones of current place,
                // say, 220, we add 210, 211, 212, 213 ... 219
                r += i;
            }
            i *= 10; // bump current place
        }
        r as i32
    }
}
