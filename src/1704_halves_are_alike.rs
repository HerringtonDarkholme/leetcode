impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s: Vec<_> = s.to_lowercase().chars().collect();
        let len = s.len();
        let is_vowel = |c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
        let first = &s[0..len/2].iter().cloned().filter(|&c| is_vowel(c)).count();
        let second = &s[len/2..].iter().cloned().filter(|&c| is_vowel(c)).count();
        first == second
    }
}
