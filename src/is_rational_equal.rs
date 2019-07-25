pub struct Solution;

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let (i1, c1) = Solution::get_integer(&s);
        let (i2, c2) = Solution::get_integer(&t);
        if i1 != i2 && (i1 - i2).abs() > 1 {
            return false
        }
        let (n1, d1) = Solution::get_rational(&s, c1);
        let (n2, d2) = Solution::get_rational(&t, c2);
        println!("{} {} {} {}", n1, d1, n2, d2);
        if i1 == i2 {
            n1 == n2 && d1 == d2
        } else if i1 > i2 {
            n1 == 0 && n2 == d2
        } else {
            n2 == 0 && n1 == d1
        }
    }

    fn get_integer(s: &[char]) -> (i32, usize) {
        let mut i = 0;
        let mut r = 0;
        for c in s {
            r += 1;
            if *c == '.' {
                break
            }
            i = i * 10 + c.to_digit(10).unwrap() as i32;
        }
        (i, r)
    }

    fn get_rational(s: &Vec<char>, mut i: usize) -> (u32, u32) {
        let mut num = 0;
        let mut den = 1;
        let mut start = i;
        for c in i..s.len() {
            if s[c] == '(' {
                start = c + 1;
                break;
            }
            num = num * 10 + s[c].to_digit(10).unwrap();
            den *= 10;
        };
        if start == i {
            let (num, den) = Solution::reduce(num, den);
            return (num, den)
        }
        let mut r_num = 0;
        let mut r_den = 0;
        for c in start..(s.len()-1) {
            r_num = r_num * 10 + s[c].to_digit(10).unwrap();
            r_den = r_den * 10 + 9;
        }
        let (r_num, r_den) = Solution::reduce(r_num, r_den);
        Solution::reduce(num * r_den + r_num, r_den * den)
    }

    fn reduce(num: u32, den: u32) -> (u32, u32) {
        let (mut a, mut b) = (num, den);
        let gcd = loop {
            if a == 0 {
                break b
            }
            let temp = a;
            a = b % a;
            b = temp;
        };
        (num / gcd, den / gcd)
    }
}

#[test]
fn test() {
    for v in vec![
        "1",
        "0.(9)",
        "1.",
        "0.9(9)",
        "1.1",
        "1.0(9)",
        "2",
        "1.(9)",
        "1.00000",
        "1",
        "0.(52)",
        "0.5(25)",
    ].chunks(2) {
        assert_eq!(true, Solution::is_rational_equal(v[0].to_owned(), v[1].to_owned()));
    }
    for v in vec![
        "0.3",
        "0.4",
        "1.01",
        "0.(9)",
    ].chunks(2) {
        assert_eq!(false, Solution::is_rational_equal(v[0].to_owned(), v[1].to_owned()));
    }
}
