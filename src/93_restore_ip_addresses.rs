impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut ret = vec![vec![s[0] - b'0']];
        for &c in s.iter().skip(1) {
            let c = c - b'0';
            let mut next = vec![];
            for mut prev in ret {
                // append last
                let last = prev[prev.len() - 1];
                if last != 0 && (last as i32 * 10 + c as i32) < 256 {
                    let mut new = prev.clone();
                    new[prev.len() - 1] = last * 10 + c;
                    next.push(new);
                }
                // push new
                if prev.len() < 4 {
                    prev.push(c);
                    next.push(prev);
                }
            }
            ret = next;
        }
        ret
            .into_iter()
            .filter(|s| s.len() == 4)
            .map(|s| format!("{}.{}.{}.{}", s[0], s[1], s[2], s[3]))
            .collect()
    }
}
// 25525511135
// [2], [[2, 5], [25]], [ [2,5,5], [25, 5], [2,55], [255]]
// 
