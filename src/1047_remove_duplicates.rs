impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut ret = vec![];
        for c in s.chars() {
            if let Some(&cc) = ret.last() {
                if cc == c {
                    ret.pop();
                    continue;
                }
            }
            ret.push(c);
        }
        ret.into_iter().collect()
    }
}
