use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let map2: HashMap<_, _> = list2.into_iter().enumerate().map(|(i, k)| (k, i)).collect();
        let mut min = usize::MAX;
        let mut ret = vec![];
        for (i, key) in list1.into_iter().enumerate() {
            if let Some(j) = map2.get(&key) {
                if i + j < min {
                    ret.clear();
                    min = i + j;
                    ret.push(key);
                } else if i + j == min {
                    ret.push(key);
                }
            }
        }
        ret
    }
}
