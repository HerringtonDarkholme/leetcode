impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let prefix = get_prefix(s);
        let mut ret = vec![0]; // how many beautiful nums
        for i in 0..s.len() {
            let mut part = -1;
            for start in 0..=i {
                // is not beautiful
                if ret[start] < 0 || s[start] == b'0' {
                    continue;
                }
                let num = if start == 0 {
                    prefix[i]
                } else {
                    (prefix[start - 1] << (i - start +1)) ^ prefix[i]
                };
                if is_beautiful(num) {
                    part = if part == -1 { ret[start] + 1} else {
                        part.min(ret[start] + 1)
                    }
                }
            }
            ret.push(part);
        }
        ret[s.len()]
    }
}
fn get_prefix(s: &[u8]) -> Vec<i32> {
    let mut prefix = vec![];
    let mut num = 0;
    for &b in s {
        num = num * 2 + (b - b'0') as i32;
        prefix.push(num);
    }
    prefix
}

fn is_beautiful(mut n: i32) -> bool {
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}
       
//   1 0 1 1
//   1 2 5 11
//   0 1 -1 1 2
