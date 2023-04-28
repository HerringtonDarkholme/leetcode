impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let strs = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let mut parents: Vec<_> = (0..strs.len()).collect();
        for i in 1..strs.len() {
            for j in 0..i {
                if is_same(&strs[i], &strs[j]) {
                    union(i, j, &mut parents);
                }
            }
        }
        let mut ret = 0;
        for i in 0..parents.len() {
            if i == parents[i] {
                ret += 1;
            }
        }
        ret
    }
}

fn is_same(a: &[u8], b: &[u8]) -> bool {
    let mut i = 0;
    for (&a, &b) in a.iter().zip(b.iter()) {
        if a != b {
            i += 1;
        }
        if i > 2 {
            return false;
        }
    }
    i <= 2 
}

fn find(mut a: usize, parents: &[usize]) -> usize {
    while a != parents[a] {
        a = parents[a];
    }
    a
}

fn union(a: usize, b: usize, parents: &mut [usize]) {
    let a = find(a, parents);
    let b = find(b, parents);
    if a < b {
        parents[a] = b;
    } else {
        parents[b] = a;
    }
}



use std::collections::HashSet;
impl Solution {
    pub fn num_similar_groups(a: Vec<String>) -> i32 {
        let mut group = 0;
        let mut visited = vec![false; a.len()];
        let mut a: HashSet<_> = a.into_iter().collect();
        let a: Vec<Vec<_>> = a.into_iter()
            .map(|s| s.chars().collect())
            .collect();
        for index in 0..a.len() {
            if visited[index] {
                continue;
            }
            group += 1;
            let mut stack = vec![index];
            while !stack.is_empty() {
                let i = stack.pop().unwrap();
                visited[i] = true;
                let s1 = &a[i];
                for j in 0..a.len() {
                    let s2 = &a[j];
                    if visited[j] || !is_similar(s1, s2) {
                        continue;
                    }
                    stack.push(j);
                }
            }
        }
        group
    }
}
fn is_similar(s1: &[char], s2: &[char]) -> bool {
    let s2 = &a[j];
    let mut i = 0;
    let mut swap = 0;

    while i < s1.len() && s1[i] == s2[i] {
        i += 1;
    }
    if i == s1.len() {
        return true
    }
    swap = i;
    i += 1;
    while i < s1.len() && s1[i] == s2[i] {
        i += 1;
    }
    if i == s1.len() || s1[swap] != s2[i] || s1[i] != s2[swap] {
        return false
    }
    i += 1;
    while i < s1.len() && s1[i] == s2[i] {
        i += 1;
    }
    i == s1.len()
}
