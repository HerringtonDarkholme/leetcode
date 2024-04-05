impl Solution {
    pub fn make_good(s: String) -> String {
        good(s)
    }
}
fn good(s: String) -> String {
    let mut ret = vec![];
    for c in s.chars() {
        let Some(&last) = ret.last() else {
            ret.push(c);
            continue;
        };
        if last.to_lowercase().next() == c.to_lowercase().next() && last.is_uppercase() != c.is_uppercase() {
            ret.pop();
            continue;
        } else {
            ret.push(c);
        }
    }
    ret.into_iter().collect()
}
