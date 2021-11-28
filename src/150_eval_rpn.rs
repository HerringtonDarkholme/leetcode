impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        assert!(!tokens.is_empty());
        macro_rules! push {
            ($op: tt) => {{
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let r = b $op a; 
                stack.push(r);
            }}
        }
        for token in tokens {
            match token.as_str() {
                "+" => push!(+),
                "-" => push!(-),
                "*" => push!(*),
                "/" => push!(/),
                d => stack.push(d.parse::<i32>().unwrap()),
            }
        }
        stack[0]
    }
}
