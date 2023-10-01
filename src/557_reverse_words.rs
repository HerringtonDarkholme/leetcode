impl Solution {
    pub fn reverse_words(s: String) -> String {
        let ret: Vec<_> = s.split(' ').map(|c|  {
            c.chars().rev().collect::<String>()
        }).collect();
        ret.join(" ")
    }
}
