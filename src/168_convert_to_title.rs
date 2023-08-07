impl Solution {
    pub fn convert_to_title(mut num: i32) -> String {
        let mut ret = vec![];
        while num != 0 {
            let c = ((num - 1) % 26) as u8 + b'A';
            ret.push(c as char);
            num = (num - 1) / 26;
        }
        ret.into_iter().rev().collect()
    }
}
