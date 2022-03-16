impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut to_remove = vec![];
        let mut ret = vec![];
        for c in s.chars() {
            if c == '(' {
                to_remove.push(ret.len());
            }
            if c != ')' || to_remove.pop().is_some() {
                ret.push(c);
            }
        }
        while let Some(i) = to_remove.pop() {
            ret.remove(i);
        }
        ret.into_iter().collect()
    }
}
