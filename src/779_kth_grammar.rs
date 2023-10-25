impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        (k - 1).count_ones() as i32 % 2 
    }
}

//   0             0
// 0  1           01
// 01 10         10 11
// 0110 1001
