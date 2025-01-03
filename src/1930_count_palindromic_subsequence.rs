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

use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut pos = vec![(-1, -1); 26];
        for (i, b) in s.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            if pos[idx].0 < 0 {
                pos[idx].0 = i as i32;
            } else {
                pos[idx].1 = i as i32;
            }
        }
        let bytes = s.as_bytes();
        let mut ret = 0;
        for  (s, e) in pos {
            if s < 0 || e < 0 {
                continue;
            }
            let mut set = HashSet::new();
            for j in s+1..e {
                set.insert(bytes[j as usize]);
            }
            ret += set.len();
        } 
        ret as i32
    }
}

#[derive(Clone, Copy, Debug)]
enum CharState {
    Unmet,
    Met {
        between: [i32; 26],
        last: i32,
    }
}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut states = [CharState::Unmet; 26];
        for (i, b) in s.bytes().enumerate() {
            let c = (b - b'a') as usize;
            for d in 0..26 {
                if c == d {
                    match states[c] {
                        CharState::Unmet => {
                            states[c] = CharState::Met {
                                between: [-1; 26],
                                last: -1,
                            }
                        }
                        CharState::Met {ref mut between, ref mut last } => {
                            if between[c] < 0 {
                                between[c] = i as i32;
                            }
                            *last = i as i32;
                        }
                    }
                } else {
                    match states[d] {
                        CharState::Met {ref mut between, .. } if between[c] < 0 => {
                                between[c] = i as i32;
                        }
                        _ => {}
                    }
                }
            }
        }
        let mut ret = 0;
        for state in states {
            match state {
                CharState::Met { between, last } if last >= 0 => {
                    for n in between {
                        if n >= 0 && n < last {
                            ret += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        ret
    }
}
