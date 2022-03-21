use std::collections::HashSet;

// convert to hashset
// generate string
// test it in nums
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let len = nums.len();
        let set: HashSet<_> = nums.into_iter().collect();
        let mut s = String::new();
        generate(&mut s, len, &set);
        s
    }
}

fn generate(s: &mut String, len: usize, set: &HashSet<String>) -> bool {
    if s.len() == len {
        return !set.contains(s);
    }
    s.push('0');
    if generate(s, len, set) {
        return true;
    }
    s.remove(s.len() - 1);
    s.push('1');
    if generate(s, len, set) {
        return true;
    }
    s.remove(s.len() - 1);
    return false;
}
