pub struct Solution;

impl Solution {
    // @param words: a list of words
    // @return: a string which is correct order
    pub fn alien_order(words: Vec<String>) -> String {
        // Write your code here
        let mut deps = vec![-1; 26];
        let words: Vec<Vec<_>> =
            words.into_iter().map(|w| w.bytes().collect()).collect();
        for w in &words {
            for &c in w {
                deps[(c - b'a') as usize] = 0;
            }
        }
        for i in 1..words.len() {
            compute_order(&words[i - 1], &words[i], &mut deps);
        }
        let mut sorted = vec![];
        for c in 0..26 {
            if deps[c] >= 0 {
                if !topological_sort(c, &deps, &mut sorted, &mut 0) {
                    return String::new();
                }
            }
        }
        sorted.into_iter().map(|b| (b as u8 + b'a') as char).collect()
    }
}

fn compute_order(w1: &[u8], w2: &[u8], deps: &mut Vec<i32>) {
    let len = w1.len().min(w2.len());
    for i in 0..len {
        let c1 = (w1[i] - b'a') as usize;
        let c2 = (w2[i] - b'a') as usize;
        if c1 == c2 {
            continue;
        }
        // c2 > c1, push c1 to c2's deps
        deps[c2] |= 1 << c1;
    }
}
fn topological_sort(c: usize, deps: &Vec<i32>, sorted: &mut Vec<usize>, visited: &mut i32) -> bool {
    if (1 << c) & *visited != 0 {
        return false;
    }
    if sorted.contains(&c) {
        return true;
    }
    *visited |= 1 << c;
    for p in 0..26 {
        if (1 << p) & deps[c] != 0 {
            if !topological_sort(p, deps, sorted, visited) {
                return false;
            }
        }
    }
    sorted.push(c);
    *visited != 1 << c;
    true
}
