impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut diff = vec![0; s.len() + 1]; // add dummy for branchless
        for shift in shifts {
            let d = shift[2] * 2 - 1;
            diff[shift[0] as usize] += d;
            diff[shift[1] as usize + 1] -= d;
        }
        let mut ret = String::with_capacity(s.len());
        let mut num_of_shift = 0;
        let s = s.as_bytes();
        diff.pop(); // remove last dummy
        for (i, d) in diff.into_iter().enumerate() {
            num_of_shift += d % 26;
            num_of_shift += 26;
            num_of_shift %= 26;
            let c = (s[i] - b'a' + num_of_shift as u8) % 26 + b'a';
            ret.push(c as char);
        }
        ret
    }
}
