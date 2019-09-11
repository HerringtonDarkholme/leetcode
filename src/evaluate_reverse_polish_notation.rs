pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for t in tokens {
            let t: &str = &t;
            match t {
                "+" | "-" | "*" | "/" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    if t == "+" {
                        stack.push(op1 + op2);
                    }
                    if t == "-" {
                        stack.push(op1 - op2);
                    }
                    if t == "*" {
                        stack.push(op1 * op2);
                    }
                    if t == "/" {
                        stack.push(op1 / op2);
                    }
                },
                c => {
                    stack.push(c.parse().unwrap());
                }
            }
        }
        stack[0]
    }
}
