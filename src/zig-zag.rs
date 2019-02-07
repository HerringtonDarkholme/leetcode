struct Solution {}

impl Solution {
    fn convert(s: String, num_row: i32) -> String {
        let num_row = num_row as usize;
        if num_row == 1 {
            return s
        }
        let chars: Vec<_> = s.chars().collect();
        let len = chars.len();
        let mut ret = Vec::with_capacity(len);
        let cycle = num_row * 2 - 2;
        for i in 0..num_row {
            let mut j = i;
            let mut yinyang = true;
            while j < len {
                ret.push(chars[j].to_string());
                let add = if yinyang { cycle - 2 * i } else { 2 * i };
                if add == 0 {
                    j += cycle;
                } else {
                    yinyang = !yinyang;
                    j += add;
                }
            }
        }
        ret.join("")
    }
}

#[test]
fn test() {
    let tests = vec!(
        ("PAYPALISHIRING", 2),
        ("PAYPALISHIRING", 3),
        ("PAYPALISHIRING", 4),
    );
    for (test, num) in tests {
        println!("{}: {}", test, Solution::convert(test.into(), num));
    }
}
