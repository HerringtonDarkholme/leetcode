impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ret = vec![];
        let (a, b) = if a.len() > b.len() {
            (a, b)
        } else {
            (b, a)
        };
        let a: Vec<_> = a.bytes().rev().collect();
        let mut b: Vec<_> = b.bytes().rev().collect();
        b.extend(vec![b'0'; a.len() - b.len()]);
        let mut carry = 0;
        for (ac, bc) in a.into_iter().zip(b.into_iter()) {
            let n = (ac - b'0') + (bc - b'0') + carry;
            carry = n / 2;
            ret.push((b'0' + n % 2) as char);
        }
        if carry > 0 {
            ret.push('1');
        }
        ret.into_iter().rev().collect()
    }
}
