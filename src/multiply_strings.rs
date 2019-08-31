impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if &num1 == "0" || &num2 == "0" {
            return "0".to_string()
        }
        let n1: Vec<i32> = num1.chars().map(|c| c as i32 - '0' as i32).collect();
        let n2: Vec<i32> = num2.chars().map(|c| c as i32 - '0' as i32).collect();
        let mut rets = vec![];
        let mut i = n2.len();
        while i > 0 {
            i -= 1;
            let mut v = vec![0; n2.len() - i - 1];
            Solution::multi_one_digit(&n1, n2[i], &mut v);
            rets.push(v);
        }
        let max_len = rets.last().unwrap().len();
        let mut ret = vec![];
        let mut carry = 0;
        for i in 0..max_len {
            let mut n = carry;
            for v in rets.iter() {
                n += *v.get(i).unwrap_or(&0);
            }
            ret.push(n % 10);
            carry = n / 10;
        }
        if carry > 0 {
            ret.push(carry);
        }
        ret.iter().rev().map(|n| n.to_string()).collect()
    }
    fn multi_one_digit(n1: &Vec<i32>, n: i32, out: &mut Vec<i32>){
        let mut carry = 0;
        let mut i = n1.len();
        while i > 0 {
            i -= 1;
            let num = n * n1[i] + carry;
            out.push(num % 10);
            carry = num / 10;
        }
        if carry > 0 {
            out.push(carry);
        }
    }
}
