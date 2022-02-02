/*
438. Find All Anagrams in a String
Medium

Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.



Example 1:

Input: s = "cbaebabacd", p = "abc"
Output: [0,6]
Explanation:
The substring with start index = 0 is "cba", which is an anagram of "abc".
The substring with start index = 6 is "bac", which is an anagram of "abc".

Example 2:

Input: s = "abab", p = "ab"
Output: [0,1,2]
Explanation:
The substring with start index = 0 is "ab", which is an anagram of "ab".
The substring with start index = 1 is "ba", which is an anagram of "ab".
The substring with start index = 2 is "ab", which is an anagram of "ab".



Constraints:

    1 <= s.length, p.length <= 3 * 104
    s and p consist of lowercase English letters.
*/

use std::collections::HashMap;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let pattern = build_pattern(p);
        find_anagrams(s, pattern)
    }
}
fn find_anagrams(s: String, pattern: HashMap<char, usize>) -> Vec<i32> {
        let mut ret = vec![];
        let mut occ = pattern.clone();
        let mut i = 0;
        let s: Vec<_> = s.chars().collect();
        for j in 0..s.len() {
            if let Some(&cnt) = occ.get(&s[j]) {
                if cnt == 0 {
                    // shrink i
                    while occ[&s[j]] == 0 {
                        *occ.get_mut(&s[i]).unwrap() += 1;
                        i += 1;
                    }
                }
                *occ.get_mut(&s[j]).unwrap() -= 1;
                if all_found(&occ) {
                    ret.push(i as i32);
                }
            } else {
                // not found
                i = j + 1;
                occ = pattern.clone();
            }
        }
        ret
}
fn all_found(occ: &HashMap<char, usize>) -> bool {
    occ.values().all(|&c| c == 0)
}

fn build_pattern(p: String) -> HashMap<char, usize> {
    let mut pattern = HashMap::new();
    for c in p.chars() {
        *pattern.entry(c).or_insert(0) += 1;
    }
    pattern
}
