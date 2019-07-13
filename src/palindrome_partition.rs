pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let cs: Vec<char> = s.chars().collect();
        let mut cache = HashMap::new();
        Solution::sub_parition(&cs, 0, &mut cache)
    }
    fn sub_parition(cs: &[char], i: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(prev) = cache.get(&i) {
            return *prev
        }
        if i == cs.len() {
            return 0
        }
        let mut min = i32::max_value();
        for j in i..cs.len() {
            if !Solution::is_palindrome(cs, i, j) {
                continue
            }
            let suffix_min = Solution::sub_parition(cs, j+1, cache);
            min = std::cmp::min(suffix_min + 1, min);
        }
        cache.insert(i, min);
        min
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
