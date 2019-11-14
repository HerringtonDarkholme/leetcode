pub struct Solution;

impl Solution {
    pub fn at_most_n_given_digit_set(mut d: Vec<String>, n: i32) -> i32 {
        let digits = count_digits(n);
        let mut ret = 0;
        let mut len = d.len();
        let mut d: Vec<i32> = d
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        for i in 1..digits {
            ret += len.pow(i as u32);
        }
        ret += count_full(&d, n, digits as u32);
        ret as i32
    }
}

fn count_digits(mut n: i32) -> i32 {
    let mut i = 0;
    while n > 0 {
        i += 1;
        n /= 10;
    }
    i
}

fn count_full(digits: &Vec<i32>, n: i32, exp: u32) -> usize {
    let mut ret = 0;
    let mut i = 0;
    let mut largest = n;
    while largest >= 10 {
        largest /= 10;
        i += 1;
    };
    if exp == 0 {
        return 1
    }
    if exp - 1 > i { // 1002, skip handle 0 midst
        return 0
    }
    for &c in digits {
        if c < largest {
            ret += digits.len().pow(i);

        } else if c == largest {
            ret += count_full(digits, n % 10i32.pow(i), i);
        } else {
            break;
        }
    }
    ret
}
