impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
       let mut map = std::collections::HashMap::new();
        for i in secret.chars() {
            *map.entry(i).or_insert(0) += 1;
        }
        let secret: Vec<_> = secret.chars().collect();
        let mut bull = 0;
        let mut cow = 0;
        for (i, c) in guess.chars().enumerate() {
            if c == secret[i] {
                bull += 1;
            }
            let e = map.entry(c).or_insert(0);
            if *e >= 1 {
                cow += 1;
                *e -= 1;
            }
        }
        format!("{}A{}B", bull, cow - bull)
    }
}
