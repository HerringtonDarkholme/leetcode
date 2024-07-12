impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut stack = vec![];
        let mut chars: Vec<_> = s.chars().collect();
        let mut ret = 0;
        if x > y {
            ret += remove('b', chars, &mut stack, x);
            ret += remove('a', stack, &mut vec![], y);
        } else {
            ret += remove('a', chars, &mut stack, y);
            ret += remove('b', stack, &mut vec![], x);
        }
        ret
    }
}
fn remove(target: char, chars: Vec<char>, stack: &mut Vec<char>, score: i32) -> i32 {
    let mut ret = 0;
    for c in chars {
        if stack.is_empty() {
            stack.push(c);
            continue;
        }
        if target == c {
            let prev = if c == 'a' {'b'} else {'a'};
            if *stack.last().unwrap() == prev {
                stack.pop();
                ret += score;
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }
    ret
}
// abab
