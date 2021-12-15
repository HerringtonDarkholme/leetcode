impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        longest(s)   
    }
}

fn longest(s: String) -> i32 {
    let mut stack = vec![];
    let mut left = 0;
    let mut current = 0;
    let mut max = 0;
    for c in s.bytes() {
        if c == b'(' {
            if current == 0 {
                left += 1;
            } else {
                stack.push((left, current));
                left = 1;
                current = 0;
            }
        } else {
            if current == left {
                if stack.is_empty() {
                    left = 0;
                    current = 0;
                } else {
                    let (p_left, r) = stack.pop().unwrap();
                    left += p_left;
                    current += 1 + r;
                    max = max.max(current); 
                }
            } else {
                current += 1;
                if current == left && !stack.is_empty() {
                    let (p_left, r) = stack.pop().unwrap();
                    left += p_left;
                    current += r;
                }
                max = max.max(current);
            }
        }
    }
    max * 2
}
