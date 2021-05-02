impl Solution {
    pub fn split_string(s: String) -> bool {
        let v: Vec<_> = s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut n = 0;
        for i in 0..(v.len()-1) {
            n = n * 10 + v[i];
            if n != 0 && has_descending(n, i+1, &v) {
                return true
            }
        }
        false
    }
}

fn has_descending(n: u64, i: usize, v: &Vec<u64>) -> bool {
    if i >= v.len() {
        return true
    }
    if n == 0 {
        return v[i..].iter().all(|&e| e ==0)
    }
    let mut m = 0;
    for j in i..v.len() {
        m = m * 10 + v[j];
        if m > n - 1 {
            break;
        }
        if m == n - 1 {
            return has_descending(m, j+1, v)
        }
    }
    false
}
