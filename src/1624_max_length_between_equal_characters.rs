impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut start = vec![-1; 26];
        let mut end = vec![-1; 26];
        for (i, c) in s.bytes().enumerate() {
            let index = (c - b'a') as usize;
            if start[index] == -1 {
                start[index] = i as i32;
                continue;
            }
            end[index] = i as i32;
        }
        let mut max = -1;
        for i in 0..26 {
            if end[i] < 0 {
                continue;
            }
            max = max.max(end[i] - start[i] - 1);
        }
        max
    }
}
