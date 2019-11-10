pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for s in cpdomains {
            let sp: Vec<&str> = s.split(' ').collect();
            let n: usize = sp[0].parse().unwrap();
            let mut domain = sp[1];
            while !domain.is_empty() {
                *map.entry(domain.to_string()).or_insert(0) += n;
                let mut split = domain.splitn(2, '.');
                split.next();
                domain = split.next().unwrap_or(&"");
            }
        }
        map.into_iter().map(|(k, v)| format!("{} {}", v, k)).collect()
    }
}
