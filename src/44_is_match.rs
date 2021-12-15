use std::collections::HashSet;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut frontier = HashSet::new();
        frontier.insert(0);
        let p = normalize(p);
        is_match(s.as_bytes(), &p, frontier)
    }
}
fn normalize(p: String) -> Vec<u8> {
    if p.is_empty() {
        return vec![];
    }
    let p = p.as_bytes();
    let mut r = vec![p[0]];
    for i in 1..p.len() {
        if p[i] == b'*' && r[r.len() - 1] == b'*' {
            continue;
        }
        r.push(p[i]);
    }
    return r
}

fn is_match(s: &[u8], p: &[u8], frontier: HashSet<usize>) -> bool {
    if s.is_empty() {
        return frontier.contains(&p.len()) || (
            *p.last().unwrap_or(&b'_') == b'*' && frontier.contains(&(p.len() - 1))
        )
    }
    let c = s[0];
    let mut next = HashSet::new();
    for i in frontier {
        if i == p.len() {
            continue;
        }
        let pc = p[i];
        if pc == c || pc == b'?' {
            next.insert(i + 1);
            continue;
        } else if pc != b'*' {
            continue;
        }
        next.insert(i);
        next.insert(i + 1);
        if i + 1 < p.len() && (p[i + 1] == c || p[i + 1] == b'?') {
            next.insert(i + 2);
        }
    }
    is_match(&s[1..], p, next)
}
