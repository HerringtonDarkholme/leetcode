impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut idx = vec![-1; 26];
        for (j, b) in s.bytes().enumerate() {
            let i = (b - b'a') as usize;
            if idx[i] == - 1 {
                idx[i] = j as i32; 
            } else {
                idx[i] = -2;
            }
        }
        let mut i = -1;
        for n in idx {
            if n >= 0 {
                if i < 0 {
                    i = n;
                } else {
                    i = i.min(n);
                }
            }
        }
        i
    }
}
