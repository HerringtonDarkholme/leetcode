pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();
        s.chunks(2 * k).map(|cs| {
            let s1: String = {
                let mut s1 = cs.iter().take(k).collect::<Vec<_>>();
                s1.reverse();
                s1.into_iter().collect()
            };
            let s2: String = cs.iter().skip(k).collect();
            format!("{}{}", s1, s2)
        }).collect::<Vec<_>>().join("")
    }
}
