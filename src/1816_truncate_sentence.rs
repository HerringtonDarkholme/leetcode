impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split(" ").collect::<Vec<_>>()[..k as usize].join(" ")
    }
}
