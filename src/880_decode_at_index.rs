impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        decode(s.as_bytes(), k as i64)
    }
}

fn decode(s: &[u8], k: i64) -> String {
    let mut count = 0;
    let mut i = 0;
    let mut last_cnt = 0;
    let mut last_b = 0;
    while count < k {
        last_cnt = count;
        let b = s[i];
        // is letter
        if b >= b'a' && b <= b'z' {
            count += 1;
            last_b = i;
        } else {
            count *= (b - b'0') as i64;
        }
        i += 1;
    }
    if count == k ||  k % last_cnt == 0 {
        (s[last_b] as char).to_string()
    } else {
        decode(s, k % last_cnt)
    }
}
