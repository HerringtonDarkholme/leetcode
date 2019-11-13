struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<_>= s.chars().collect();
        if s.is_empty() {
            return "".into()
        }
        let mut ret = (0, 0);
        for p in 0..s.len() {
            for diff in vec![0, 1] {
                let mut i = p;
                let mut j = p + diff;
                if p + diff >= s.len() || s[i] != s[j] {
                    continue;
                }
                loop {
                    if i == 0 || j == s.len() - 1 || s[i-1] != s[j+1] {
                        break;
                    }
                    i -= 1;
                    j += 1;
                }
                if j - i > ret.1 - ret.0 {
                    ret = (i, j);
                }
            }

        }
        s[ret.0..=ret.1].iter().collect()
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
