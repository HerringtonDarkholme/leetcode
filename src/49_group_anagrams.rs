pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let prime: [u128;26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];
        let mut hashmap = std::collections::HashMap::new();
        for s in strs.into_iter() {
            let mut hash = 1;
            for c in s.chars() {
                hash *= prime[c as usize - 'a' as usize];
            }
            hashmap.entry(hash)
                .or_insert_with(|| Vec::new())
                .push(s);
        }
        hashmap.into_iter().map(|(_, v)| v).collect()
    }
}

#[test]
fn test() {
    use super::util::test::test_cases;
    let cases = vec![
        // (
        //     vec_str!["eat", "tea", "tan", "ate", "nat", "bat"],
        //     vec![
        //       vec_str!["tan", "nat"],
        //       vec_str!["eat","tea", "ate"],
        //       vec_str!["bat"]
        //     ]
        // ),
        (vec_str!["zzzzzzzzzzzzzzzzzz"], vec![vec_str!["zzzzzzzzzzzzzzzzzz"]])
    ];
    test_cases(cases, Solution::group_anagrams);
}
