impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut i: Vec<_> = s.bytes().collect();
        let mut ret = 0;
        let mut carry = false;
        while i.len() > 1 {
            let len = i.len();
            let last = i[len - 1];
            if carry {
                ret += if last == b'1' { 1 } else { 2 };
            } else {
                if last == b'0' {
                    ret += 1;
                } else {
                    ret += 2;
                    carry = true;
                }
            }
            i.pop();
        }
        if carry { ret + 1 } else { ret }
    }
}
