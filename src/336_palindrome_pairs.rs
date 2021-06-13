impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut words = words.into_iter()
            .map(|v| v.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut helper = words.iter().map(|w| {
            let mut count = [0; 26];
            for &c in w.iter() {
                count[c as usize - 'a' as usize] += 1;
            }
            (w.len(), count)
        }).collect::<Vec<_>>();
        let mut ret = vec![];
        for i in 0..words.len() {
            for j in i+1..words.len() {
                let a = &helper[i].1;
                let b = &helper[j].1;
                let mut pass = true;
                if helper[i].0 > helper[j].0 {
                    for u in 0..26 {
                        if a[u] < b[u] {
                            pass = false;
                            break;
                        }
                    }
                } else {
                    for u in 0..26 {
                        if a[u] > b[u] {
                            pass = false;
                            break;
                        }
                    }
                }
                if !pass {
                    continue;
                }
                let mut s1 = words[i].clone();
                s1.extend(&words[j]);
                let mut s2 = words[j].clone();
                s2.extend(&words[i]);
                if is_palindrome(s1) {
                    ret.push(vec![i as i32, j as i32]);
                }
                if is_palindrome(s2) {
                    ret.push(vec![j as i32, i as i32]);
                }
            }
        }
        ret
    }
}

fn is_palindrome(s: Vec<char>) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false
        }
        i += 1;
        j -= 1;
    }
    true
}
