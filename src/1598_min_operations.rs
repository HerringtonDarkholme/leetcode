impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut i = 0;
        for log in logs {
            if log == "../" {
                i = 0.max(i - 1);
            } else if log != "./" {
                i += 1;
            }
        }
        i
    }
}
