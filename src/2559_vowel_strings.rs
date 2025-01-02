impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = vec![0];
        let mut sum = 0;
        for word in words {
            if is_vowel_strings(word) {
                sum += 1;
            }
            prefix.push(sum);
        }
        queries.into_iter().map(|q| {
            let (s, e) = (q[0] as usize, q[1] as usize);
            prefix[e + 1] - prefix[s]
        }).collect()
    }
}

fn is_vowel_strings(s: String) -> bool {
    let chars: Vec<_> = s.chars().collect();
    matches!(chars[0], 'a' | 'e' | 'i' | 'o' | 'u') && 
    matches!(chars[chars.len() - 1], 'a' | 'e' | 'i' | 'o' | 'u')
}
