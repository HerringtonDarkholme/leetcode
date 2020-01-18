impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let v = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let codes: std::collections::HashSet<_> = 
            words.into_iter().map(|s| str_to_code(s, &v)).collect();
        codes.len() as i32
    }
}

fn str_to_code(s: String, v: &Vec<&str>) -> String {
    s.chars().map(|c| {
        v[(c as u8 - 'a' as u8) as usize]
    }).collect()
}
