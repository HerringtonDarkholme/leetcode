pub struct Soution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut stack = vec![];
        let mut index = vec![];
        let mut sign = 1;
        let mut num = 0;
        let chars = s.chars().collect::<Vec<_>>();
        for k in 0..chars.len() {
            let c = chars[k];
            match c {
                '[' => {
                    index.push(stack.len());
                },
                ']' => {
                    let mut v = vec![];
                    let mut i = index.pop().unwrap();
                    while stack.len() > i {
                        v.push(stack.pop().unwrap());
                    }
                    v.reverse();
                    stack.push(NestedInteger::List(v));
                },
                '-' => {
                    sign = -1;
                },
                ',' => {},
                _ => {
                    num = num * 10 + (c as i32 - '0' as i32);
                    let end = if k + 1 >= chars.len() {
                        true
                    } else {
                        match chars[k+1] {
                            '0'..='9' => false,
                            _ => true,
                        }
                    };
                    if end {
                        stack.push(NestedInteger::Int(num * sign));
                        num = 0;
                        sign = 1;
                    }
                }
            }
        }
        stack.pop().unwrap()
    }
}
