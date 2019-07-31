pub struct Solution;

use std::collections::BTreeMap;

#[derive(Eq, PartialEq)]
enum State {
    InName,
    InParen,
    InCloseParen,
    InNameNumber,
    InParenNumber,
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut map = BTreeMap::new();
        let chars: Vec<char> = formula.chars().collect();
        Solution::analyze_one_paren(&chars, 0, &mut map);
        map.into_iter()
            .map(|(k, v)| if v > 1  { format!("{}{}", k, v) } else { k })
            .collect::<Vec<String>>()
            .join("")
    }

    fn analyze_one_paren(chars: &[char], mut i: usize, map: &mut BTreeMap<String, u32>) -> usize {
        let mut state = State::InParen;
        let mut sub_map: BTreeMap<String, u32> = BTreeMap::new();
        let mut name = "".to_owned();
        let mut count = 0;
        let len = chars.len();
        while i < len {
            let c = chars[i];
            match c {
                'A'..='Z' => {
                    match state {
                        State::InCloseParen | State::InParenNumber => {
                            break;
                        },
                        State::InName | State::InNameNumber => {
                            *sub_map.entry(name.clone()).or_insert(0) += count;
                        },
                        State::InParen => {},
                    }
                    name.clear();
                    name.push(c);
                    count = 1;
                    state = State::InName;
                },
                'a'..='z' if state == State::InName => {
                    name.push(c);
                },
                '0'..='9' => {
                    let cnt = c.to_digit(10).unwrap();
                    match state {
                        State::InName => {
                            state = State::InNameNumber;
                            count = cnt;
                        },
                        State::InCloseParen => {
                            state = State::InParenNumber;
                            count = cnt;
                        },
                        State::InParenNumber | State::InNameNumber => {
                            count = count * 10 + cnt;
                        },
                        State::InParen => {
                            panic!("impossilbe")
                        },
                    }
                },
                '(' => {
                    match state {
                        State::InCloseParen | State::InParenNumber => {
                            break;
                        },
                        State::InName | State::InNameNumber => {
                            *sub_map.entry(name.clone()).or_insert(0) += count;
                        },
                        State::InParen => {},
                    }
                    // compensate for the i += 1 in the looping
                    i = Solution::analyze_one_paren(chars, i + 1, &mut sub_map) - 1;
                    count = 1; //reset count if i > len
                    state = State::InParen;
                },
                ')' => {
                    match state {
                        State::InName | State::InNameNumber => {
                            *sub_map.entry(name.clone()).or_insert(0) += count;
                        },
                        State::InCloseParen | State::InParenNumber => {
                            break;
                        },
                        State::InParen => {},
                    }
                    state = State::InCloseParen;
                    count = 1;
                },
                _ => panic!("Wrong format"),
            }
            i += 1;
        }
        match state {
            State::InName | State::InNameNumber => {
                *sub_map.entry(name.clone()).or_insert(0) += count;
                count = 1; // reset count for final liquidation
            },
            _ => {}
        }
        for (k, v) in sub_map {
            *map.entry(k).or_insert(0) += v * count;
        }
        i
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;

    for i in vec![
        "Mg(OH)2",
        "((O)2)2",
        "K4(ON(SO3)2)2",
        "H2O",
        "(H2O2)",
        "(H2O2)3",
        "((O)3)2",
        "((O))2",
        "(O)(O)",
        "(O)(O)(H)",
        "(H)2(O)1",
        "((N)1(O)1(C)1)",
        "((N42)24(OB40Li30CHe3O48LiNN26)33(C12Li48N30H13HBe31)21(BHN30Li26BCBe47N40)15(H5)16)14",
        "Be32",
        "H3Be32",
    ] {
        println!("{}", Solution::count_of_atoms(i.to_string()));
    }
}
