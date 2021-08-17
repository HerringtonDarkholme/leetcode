impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut n1: Vec<_> = num1.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut n2: Vec<_> = num2.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut ret = vec![];
        let mut carry = 0;
        loop {
            let (a, b) = (n1.pop(), n2.pop());
            if a.is_none() && b.is_none() {
                break;
            }
            let n = a.unwrap_or(0) + b.unwrap_or(0) + carry;
            carry = n / 10;
            ret.push((n % 10).to_string());
        }
        if carry > 0 { 
            ret.push("1".to_string());
        }
        ret.reverse();
        ret.join("")
    }
}
