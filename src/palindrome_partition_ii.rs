pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let cs: Vec<char> = s.chars().collect();
        let mut cache = HashMap::new();
        Solution::sub_parition(&cs, 0, &mut cache)
    }
    fn sub_parition(cs: &[char], i: usize, cache: &mut HashMap<usize, Vec<Vec<String>>>) -> Vec<Vec<String>> {
        if let Some(prev) = cache.get(&i) {
            return prev.clone()
        }
        if i == cs.len() {
            return vec![vec![]]
        }
        let mut ret = vec![];
        for j in i..cs.len() {
            if !Solution::is_palindrome(cs, i, j) {
                continue
            }
            let prefix = (&cs[i..j+1]).iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
            let mut sub = Solution::sub_parition(cs, j+1, cache);
            for v in sub.iter_mut() {
                v.insert(0, prefix.clone());
            }
            ret.extend(sub);
        }
        cache.insert(i, ret.clone());
        ret
    }
    fn is_palindrome(cs: &[char], mut i: usize, mut j: usize) -> bool {
        while i < j {
            if cs[i] != cs[j] {
                return false
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[test]
fn test() {
    let a = "aabb".to_string();
    println!("{:?}", Solution::partition(a));
}
