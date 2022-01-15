const M: i64 = 1_000_000_007;
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        // maintain a record of how many As and trailing Ls
        let mut record = [
            1, // 0 A, 0 L
            1, // 0 A, 1 L
            0, // 0 A, 2 L
            1, // 1 A, 0 L
            0, // 1 A, 1 L
            0, // 1 A, 2 L
        ];
        for _ in 1..n {
            record = [
              (record[0] + record[1] + record[2]) % M, // insert P after 0As
              record[0], // insert L after 0A 0L
              record[1], // insert L after 0A 1L
              (
                record[0] + record[1] + record[2] + // insert A after 0As
                record[3] + record[4] + record[5]   // insert P after 1As
              ) % M,
              record[3], // insert L after 1A 0L
              record[4], // insert L after 1A 1L
            ];
        }
        let sum: i64 = record.iter().sum();
        (sum % M) as i32
    }
}
