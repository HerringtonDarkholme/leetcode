struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let cs: Vec<_> = s.chars().collect();
        let mut i = 0;
        use std::collections::HashMap;
        let hash: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ].into_iter().collect();

        let mut ret = 0;
        while i < cs.len() {
            let c = cs[i];
            match c {
                'I' if i < cs.len() - 1 => {
                    let n = cs[i+1];
                    if n == 'V' || n == 'X' {
                        ret += hash[&n] - 1;
                        i += 1;
                    } else {
                        ret += hash[&c];
                    }
                },
                'X' if i < cs.len() - 1 => {
                    let n = cs[i+1];
                    if n == 'L' || n == 'C' {
                        ret += hash[&n] - 10;
                        i += 1;
                    } else {
                        ret += hash[&c];
                    }
                },
                'C' if i < cs.len() - 1 => {
                    let n = cs[i+1];
                    if n == 'D' || n == 'M' {
                        ret += hash[&n] - 100;
                        i += 1;
                    } else {
                        ret += hash[&c];
                    }
                },
                _ => ret += hash[&c],
            }
            i += 1;
        }
        ret
    }
}
