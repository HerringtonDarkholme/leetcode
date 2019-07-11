struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut last = vec![1];
        let mut ret = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows {
            let next = Solution::generate_row(&last);
            ret.push(last);
            last = next;
        }
        ret
    }
    fn generate_row(last_row: &[i32]) -> Vec<i32> {
        let len = last_row.len();
        let mut vec = Vec::with_capacity(len + 1 as usize);
        vec.push(1);
        for i in 0..len-1 {
            vec.push(last_row[i] + last_row[i + 1]);
        }
        vec.push(1);
        vec
    }
}
