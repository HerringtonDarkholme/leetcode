impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut n2 = 1;
        let mut n1 = 1;
        for i in 1..=n {
            n2 = n1 + n2;
            n1 = n2 - n1;
        }
        n1
    }
}
