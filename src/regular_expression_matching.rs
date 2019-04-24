pub struct Solution;

use std::str::FromStr;

#[derive(Debug)]
enum Pattern {
    Char {
        char: char,
        repeat: bool,
    },
    Dot {
        repeat: bool,
    }
}

#[derive(Debug)]
struct NFA {
    pattern: Pattern,
    next: Option<Box<NFA>>,
}

impl NFA {
    fn match_str(&self, s: &[char]) -> bool {
        match self.pattern {
            Pattern::Char {char, repeat} => {
                if let Some(n) = self.next.as_ref() {
                    if !repeat {
                        if s.is_empty() || s[0] != char {
                            return false
                        }  else {
                            return n.match_str(&s[1..])
                        }
                    } else {
                        for i in 0..=s.len() {
                            if n.match_str(&s[i..]) {
                                return true
                            }
                            if s.len() == i || char != s[i] {
                                return false
                            }
                        }
                        false
                    }
                } else {
                    if !repeat {
                        s.len() == 1 && s[0] == char
                    } else {
                        s.iter().all(|c| *c == char)
                    }
                }
            },
            Pattern::Dot { repeat } => {
                if let Some(n) = self.next.as_ref() {
                    if !repeat {
                        !s.is_empty() && n.match_str(&s[1..])
                    } else {
                        for i in (0..=s.len()).rev() {
                            if n.match_str(&s[i..]) {
                                return true;
                            }
                        }
                        false
                    }
                } else {
                    if !repeat {
                        s.len() == 1
                    } else {
                        true
                    }
                }
            }
        }
    }
}

impl FromStr for NFA {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret: Option<Box<NFA>> = None;
        let mut nfa = &mut ret;
        let v: Vec<_> = s.chars().collect();
        let mut i = 0;
        while i < v.len() {
            let c = v[i];
            i += 1;
            let repeat = if i < v.len() && v[i] == '*' {
                i += 1;
                true
            } else { false };
            let pattern = match c {
                '*' => {
                    return Err(std::fmt::Error)
                },
                '.' => {
                    Pattern::Dot{repeat}
                },
                c => {
                    Pattern::Char{char: c, repeat}
                },
            };
            let next = Some(Box::new(NFA{pattern, next: None}));
            if nfa.is_some() {
                let n = nfa.as_mut().unwrap();
                n.next = next;
                nfa = &mut n.next;
            } else {
                *nfa = next;
            }
        }
        if let Some(m) = ret {
            Ok(*m)
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if let Ok(nfa) = p.parse::<NFA>() {
            let v: Vec<_> = s.chars().collect();
            nfa.match_str(&v)
        } else {
            s.is_empty() && p.is_empty()
        }
    }
}

#[test]
fn test() {
    for (s, p, e) in vec![
        ("test", "test", true),
        ("tst", "test", false),
        ("test", "t.st", true),
        ("test", "t.*st", true),
        ("tst", "t.*st", true),
        ("te1231231st", "t.*st", true),
        ("te1231231st", "t.*", true),
        ("ttttttt", "tt*", true),
        ("ttttatt", "tt*", false),
        ("ttttatt", "tt*att", true),
        ("mississippi", "mis*is*p*.", false),
        ("", "a*", true),
        ("", "a*b*", true),
        ("", ".*b", false),
        ("", "bb", false),
        ("", "**", false),
        ("", ".*", true),
        ("a", "ab*a", false),
        ("a", "b*a", true),
        ("aa", "ab*a", true),
        ("abbbba", "ab*a", true),
        ("aaa", "a*", true),
        ("aaaa", "a*a", true),
        ("a", ".*..a", false),
        ("", "", true),
        ("", "aa", false),
    ] {
        assert_eq!(e, Solution::is_match(s.to_owned(), p.to_owned()))
    }
}
