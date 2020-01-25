impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.chars().collect::<Vec<_>>();
        queries.into_iter().map(|q| {
            let q = q.chars().collect::<Vec<_>>();
            is_match(&q, &pattern)
        }).collect()
    }
}

fn is_match(query: &Vec<char>, pattern: &Vec<char>) -> bool {
    let mut i = 0;
    for j in 0..query.len() {
        if i == pattern.len() {
            if query[j].is_lowercase() {
                continue;
            } else {
                return false;
            }
        }
        if query[j] == pattern[i] {
            i += 1;
        } else if query[j].is_uppercase() {
            return false;
        }
    }
    i == pattern.len()
}
