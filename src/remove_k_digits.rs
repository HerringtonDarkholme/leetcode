pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack = vec![];
        let remain = num.len() - k as usize;
        for n in num.chars() {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > n {
                k -= 1;
                stack.pop();
            }
            stack.push(n);
        }
        let s: String = stack.into_iter()
            .take(remain)
            .skip_while(|&c| c == '0')
            .collect();
        if s.is_empty() {
            "0".to_string()
        } else {
            s
        }
    }
}
