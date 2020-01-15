impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s: Vec<_> = s.chars().collect();
        let num = &s[1..s.len() - 1];
        gen_nums(&num)
    }
}

fn gen_nums(s: &[char]) -> Vec<String> {
    let mut ret = vec![];
    for i in 1..s.len() {
        let x = &s[..i];
        let y = &s[i..];
        for i in gen_num(x).iter() {
            for j in gen_num(y).iter() {
                ret.push(format!("({}, {})", i, j));
            }
        }
    }
    ret
}

fn illegal_int(s: &[char]) -> bool {
    s.len() > 1 && s[0] == '0'
}

fn illegal_dec(s: &[char]) -> bool {
    *s.last().unwrap() == '0'
}

fn gen_num(s: &[char]) -> Vec<String> {
    let mut ret = vec![];
    if !illegal_int(s) {
        ret.push(s.iter().collect());
    }
    for i in 1..s.len() {
        let a = &s[..i];
        let b = &s[i..];
        if illegal_int(a) || illegal_dec(b) {
            continue;
        }
        ret.push(format!("{}.{}", a.iter().collect::<String>(), b.iter().collect::<String>()));
    }
    ret
}
