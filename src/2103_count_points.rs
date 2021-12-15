impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut s = [0; 10];
        for chunk in rings.bytes().collect::<Vec<_>>().chunks(2) {
            let i = (chunk[1] - b'0') as usize;
            let offset = match chunk[0] {
                b'R' => 0,
                b'G' => 1,
                b'B' => 2,
                _ => 3,
            };
            s[i] |= 1 << offset;
        }
        s.into_iter().filter(|s| **s == 0b111).count() as i32
    }
}
