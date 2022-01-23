impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ret = vec![];
        for i in 1..9 {
            let mut n = 0;
            for j in i..=9 {
                n = 10 * n + j;
                if n >= low && n <= high {
                    ret.push(n);
                }
                if n > high {
                    break;
                }
            }
        }
        ret.sort();
        ret
    }
}
