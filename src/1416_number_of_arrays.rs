const M: i64 = 1_000_000_007;
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let mut bytes = s.bytes();
        let mut cases = {
            let first_num = (bytes.next().unwrap() - b'0') as i64;
            if first_num == 0 {
                return 0;
            }
            vec![(first_num, 1)]
        };
        for b in bytes {
            let mut next = vec![];
            let mut existing = 0;
            let n = (b - b'0') as i64;
            for (prefix, cnt) in cases {
                existing += cnt;
                existing %= M;
                let new_num = prefix * 10 + n;
                if new_num <= k as i64 {
                    next.push((new_num, cnt));
                }
            }
            if n != 0 {
                next.push((n, existing));
            }
            cases = next;
        }
        let ret: i64 = cases.into_iter().map(|n| n.1).sum();
        (ret % M) as i32
    }
}
