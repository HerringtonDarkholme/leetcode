/**
    Recursively remove all adjacent duplicates
    Input: azxxzy
    Output: ay
    First “azxxzy” is reduced to “azzy”.
    The string “azzy” contains duplicates,
    so it is further reduced to “ay”.

    Input: geeksforgeeg
    Output: gksfor
    First “geeksforgeeg” is reduced to
    “gksforgg”. The string “gksforgg”
    contains duplicates, so it is further
    reduced to “gksfor”.

    Input: caaabbbaacdddd
    Output: Empty String

    Input: acaaabbbacdddd
    Output: acac *
 */

fn recursively_remove_adjacent_duplicates(s: String) -> String {
    let mut ret = vec![];
    let mut i = 0;
    let s = s.as_bytes();
    while i < s.len() {
        if ret.is_empty() || ret[ret.len() - 1] != s[i] {
            ret.push(s[i]);
            i += 1;
            continue;
        }
        while i < s.len() && ret[ret.len() - 1] == s[i] {
            i += 1;
        }
        ret.pop();
    }
    ret.into_iter().map(|b| b as char).collect()
}

mod test {
    use super::*;
    fn remove_dup(s: &str, t: &str) {
        assert_eq!(
            recursively_remove_adjacent_duplicates(s.to_owned()),
            t
        );
    }
    #[test]
    fn test_example() {
        for (s, t) in [
            ("azxxzy", "ay"),
            ("geeksforgeeg", "gksfor"),
            ("caaabbbaacdddd", ""),
            ("acaaabbbacdddd", "acac"),
        ] {
            remove_dup(s, t);
        }
    }
}
