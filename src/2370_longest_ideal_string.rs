impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut long = vec![0; 26];
        for c in s.bytes() {
            let mut max = 0;
            for i in (-k)..=k {
                let i = (c - b'a') as i32 + i;
                if i < 0 || i >= 26 { continue; }
                max = long[i as usize].max(max);
            }
            long[(c - b'a') as usize] = max + 1;
        }
        long.into_iter().max().unwrap()
    }
}
