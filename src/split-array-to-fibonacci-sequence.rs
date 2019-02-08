struct Solution {}

impl Solution {
    fn split_into_fibonacci(s: String) -> Vec<i32> {
        let chars: Vec<i32> = s.chars()
            .map(|i| i.to_string().parse().unwrap()).collect();
        let ret = Vec::new();
        let len = chars.len();
        for i in 0..(len / 2) {
            let n1 = Solution::get_num(&chars, 0, i+1);
            if n1 < 0 {
                return vec!()
            }
            for j in (i+1)..(len-i) {
                let n2 = Solution::get_num(&chars, i+1, j+1);
                if n2 < 0 {
                    continue
                }
                let ret = vec![n1, n2];
                if let Some(ret) = Solution::split(&chars, j+1, ret) {
                    return ret
                }
                if j == i+1 && n2 == 0 {
                    continue
                }
            }
            if i == 0 && n1 == 0 {
                return vec!();
            }
        }
        ret
    }

    fn split(chars: &[i32], start: usize, ret: Vec<i32>) -> Option<Vec<i32>> {
        let ret_len = ret.len();
        if chars.len() == start && ret_len > 2 {
            return Some(ret)
        }
        if ret[ret_len - 1] > i32::max_value() - ret[ret_len - 2] {
            return None
        }
        for i in start..chars.len() {
            let num = Solution::get_num(chars, start, i + 1);
            if num < 0 {
                return None
            }
            if num == ret[ret_len - 1] + ret[ret_len - 2] {
                let mut ret = ret.clone();
                ret.push(num);
                if let Some(ret) = Solution::split(chars, i + 1, ret) {
                    return Some(ret)
                }
            }
            if i == start && num == 0 {
                continue
            }
        }
        None
    }

    fn get_num(chars: &[i32], start: usize, end: usize) -> i32 {
        let cs = chars.get(start..end).unwrap();
        let mut ret = 0;
        for c in cs {
            if ret > i32::max_value() / 10 - c / 10 {
                return -1
            }
            ret = ret * 10 + c;
        }
        ret
    }
}

fn main() {
    let tests = vec![
        "123456579",
        "11235813",
        "112358130",
        "0123",
        "1101111",
        "31326395158253411",
        "539834657215398346785398346991079669377161950407626991734534318677529701785098211336528511",
    ];
    for test in tests {
        println!("{}: {:?}", test, Solution::split_into_fibonacci(test.into()));
    }
}
