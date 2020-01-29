impl Solution {
    pub fn maximum69_number (mut num: i32) -> i32 {
        let mut ret = vec![];
        while num > 0 {
            ret.push(num % 10);
            num /= 10;
        }
        let mut r = 0;
        let mut found = false;
        while !ret.is_empty() {
            let n = ret.pop().unwrap();
            if n == 6 && !found {
                found = true;
                r = r * 10 + 9;
            } else {
                r = r * 10 + n;
            }
        }
        r
    }
}
