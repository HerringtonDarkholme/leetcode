pub struct Solution;

impl Solution {
    pub fn replace_words(mut dict: Vec<String>, sen: String) -> String {
        dict.sort_by(|d1, d2| d1.len().cmp(&d2.len()));
        sen.split(" ").map(|mut w| {
            for d in dict.iter() {
                if w.starts_with(d) {
                    w = d;
                }
            }
            w
        }).collect::<Vec<_>>().join(" ")
    }
}
