const N: usize = 26;
#[derive(Clone)]
struct Info {
    met: bool,
    mid: [bool; N],
    used: [bool; N],
}

impl Info {
    fn new() -> Self {
        Self {
            met: false,
            mid: [false; N],
            used: [false; N],
        }
    }
}

// a new char should count three parts
// as the first char in a new palindrome
// as the mid char in a palindrome
// as the final char in a palindrome
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut ret = vec![Info::new(); 26];
        let mut r = 0;
        for i in s.as_bytes().into_iter().map(|b| (b-'a' as u8) as usize) {
            // run finalizer first
            let mut info = &mut ret[i];
            for j in 0..N {
                if info.mid[j] {
                    info.used[j] = true;
                    info.mid[j] = false;
                    r += 1;
                }
            }
            // then mid
            for info in ret.iter_mut() {
                if !info.met || info.used[i] {
                    continue;
                }
                info.mid[i] = true;
            }
            // finally initializer
            let mut info = &mut ret[i];
            if !info.met {
                info.met = true;
                continue;
            }

        }
        r
    }
}
