pub struct Solution;

use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: HashMap<&String, HashSet<&String>> = HashMap::new();
        let mut name_mapping: HashMap<&String, &String> = HashMap::new();
        for account in accounts.iter() {
            let name = &account[0];
            let emails = &account[1..];
            for e in emails.iter() {
                graph.entry(e).or_insert(HashSet::new()).insert(&emails[0]);
                graph.get_mut(&&emails[0]).unwrap().insert(e);
                name_mapping.insert(e, name);
            }
        }
        let mut seen: HashSet<&String> = HashSet::new();
        let mut ret = vec![];
        for &e in graph.keys() {
            if seen.contains(&e) {
                continue;
            }
            seen.insert(e);
            let mut stack = vec![e];
            let mut component = vec![];
            while !stack.is_empty() {
                let n = stack.pop().unwrap();
                component.push(n);
                for other in graph[n].iter() {
                    if seen.contains(other) {
                        continue;
                    }
                    seen.insert(other);
                    stack.push(other);
                }
            }
            let name = name_mapping[e];
            component.sort();
            component.insert(0, name);
            ret.push(component);
        }
        ret.iter().map(|v| v.into_iter().map(|s| s.to_string()).collect()).collect()
    }
}
