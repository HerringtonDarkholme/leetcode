struct Solution {}

impl Solution {
    fn longest_palindrome(s: String) -> String {
        let ss: Vec<_>= s.chars().collect();
        let len = ss.len();
        let mut max_start = 0;
        let mut max_end = 0;
        for i in 0..len {
            for j in 0..(len-i) {
                let k = len - j - 1;
                if ss[i] != ss[k] {
                    continue;
                }
                let mut s = i;
                let mut e = k;
                let mut is_palindrom = true;
                while s < e {
                    if ss[s] != ss[e] {
                        is_palindrom = false;
                        break;
                    }
                    s += 1;
                    e -= 1;
                }
                if is_palindrom && (k - i) > (max_end - max_start) {
                    max_start = i;
                    max_end = k;
                }
            }
        }
        s.get(max_start..max_end+1).unwrap().into()
    }
}

#[test]
fn test() {
    let tests = vec!("babad", "cbbd", "banana", "abbc");
    for test in tests {
        let ret = Solution::longest_palindrome(test.into());
        println!("{}: {}", test, ret);
    }
}
