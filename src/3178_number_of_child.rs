impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let remain = k % (2 * n - 2);
        if remain <= n - 1 {
            remain
        } else {
            2 * n - 2 - remain
        }
    }
}
