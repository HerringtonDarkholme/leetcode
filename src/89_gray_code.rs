impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        gray_code(n)
    }
}

fn gray_code(n: i32) -> Vec<i32> {
    let mut r = vec![0, 1];
    for i in 1..n {
        let rev: Vec<_> = r
            .iter()
            .rev()
            .map(|e| e | 1 << i)
            .collect();
        r.extend(rev);
    }
    r
}
