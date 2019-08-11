pub struct Solution;

impl Solution {
    pub fn find_ladders(begin: String, end: String, words: Vec<String>) -> Vec<Vec<String>> {
        let mut graph = std::collections::HashMap::new();
        if !words.contains(&begin) {
            for i in 0..words.len() {
                let s = &words[i];
                if Solution::is_adjacent_string(&begin, s) {
                    graph.entry(&begin).or_insert(vec![]).push(s);
                }
            }
        }
        for i in 0..words.len() {
            for j in i..words.len() {
                let s1 = &words[i];
                let s2 = &words[j];
                if Solution::is_adjacent_string(s1, s2) {
                    graph.entry(s1).or_insert(vec![]).push(s2);
                    graph.entry(s2).or_insert(vec![]).push(s1);
                }
            }
        }
        let mut visited = std::collections::HashSet::new();
        let mut frontier = vec![vec![&begin]];
        let end = &end;
        while !frontier.is_empty() {
            let mut next = vec![];
            let mut found = false;
            for path in frontier {
                let s = *path.last().unwrap();
                if !graph.contains_key(&s) {
                    continue;
                }
                for &n in graph[s].iter() {
                    if visited.contains(n) {
                        continue;
                    }
                    if n == end {
                        found = true;
                    }
                    let mut p = path.clone();
                    p.push(n);
                    next.push(p);
                }
            }
            if found {
                return next.into_iter().filter_map(|v| {
                    let last = *v.last().unwrap();
                    if last == end {
                        Some(v.into_iter().map(|s| s.to_owned()).collect())
                    } else {
                        None
                    }
                }).collect()
            } else {
                frontier = next;
                for v in frontier.iter() {
                    visited.insert(*v.last().unwrap());
                }
            }
        }
        vec![]
    }
    fn is_adjacent_string(s1: &str, s2: &str) -> bool {
        s1.chars().zip(s2.chars())
            .filter_map(|(c1, c2)| if c1 != c2 { Some(1) } else { None })
            .sum::<i32>() == 1
    }
}

#[test]
fn test() {
    // Solution::find_ladders("a".to_owned(), "c".to_owned(), nested!["a", "b", "c"]);
    Solution::find_ladders("hot".to_owned(), "dog".to_owned(), nested!["hot", "dog"]);
    // dbg!(Solution::find_ladders("hot".to_owned(), "cog".to_owned(), nested!["hot","dot","dog","lot","log","cog"]));
    panic!("sdfs");
    // Solution::find_ladders("hot".to_owned(), "cog".to_owned(), nested!["hot","dot","dog","lot","log"]);
}
