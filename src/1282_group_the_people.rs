use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut ret = vec![];
        for (i, n) in group_sizes.into_iter().enumerate() {
            let group = map.entry(n).or_insert_with(Vec::new);
            group.push(i as i32);
            if group.len() == n as usize {
                ret.push(group.drain(..).collect());
            }
        }
        ret
    }
}
