use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut next = HashSet::new();
        let mut ans: HashSet<i32> = HashSet::new();
        for i in arr {
            next.insert(i);
            next.extend(set.iter().map(|j| i|j));
            ans.extend(next.iter());
            set.clear();
            std::mem::swap(&mut next, &mut set);
        }
        ans.len() as i32
    }
}
