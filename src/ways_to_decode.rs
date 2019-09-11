pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        ways_to_decode(s) as i32
    }
}

pub fn ways_to_decode<S: AsRef<str>>(data: S) -> usize {
    let chars: Vec<_> = data.as_ref().chars().collect();
    if chars.is_empty() || chars[0] == '0' {
        return 0
    }
    let mut ways_minus_2 = 1;
    let mut ways_minus_1 = 1;
    for i in 1..chars.len() {
        let ways = match chars[i] {
            '0' => {
                if chars[i - 1] != '1' && chars[i - 1] != '2' {
                    return 0
                }
                ways_minus_2
            },
            c => {
                if chars[i - 1] == '1' || (c < '7' && chars[i - 1] == '2'){
                    ways_minus_1 + ways_minus_2
                } else {
                    ways_minus_1
                }
            },
        };
        ways_minus_2 = ways_minus_1;
        ways_minus_1 = ways;
    }
    ways_minus_1
}

#[test]
fn test() {
    for (s, expected) in vec![
        ("227", 2),
        ("12", 2),
        ("123", 3),
        ("127", 2),
        ("107", 1),
        ("1318145227198931047", 32),
        ("1234512431144253526371125631132512345324", 19440),
        ("3483987232111111111111111", 3194)
    ] {
        assert_eq!(ways_to_decode(s), expected);
    }
}
