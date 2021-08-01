impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut s = false;
        for i in 2..n {
            if n % i == 0 {
                if s {
                    return false
                }
                s = true;
            }
        }
        s
    }
}
