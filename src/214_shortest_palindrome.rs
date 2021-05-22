impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let o = s.clone();
        let s: Vec<_> = s.chars().collect();
        let r = {
            let mut s = s.clone();
            s.reverse();
            s
        };
        let i = find_index(&s, &r);
        let mut ret: String = r[..i].iter().collect();
        ret.push_str(&o);
        ret
    }
}

fn find_index(s: &[char], r: &[char]) -> usize {
    let table = kmp_table(&s);
    let mut j = 0;
    let mut k = 0;
    while j < r.len() {
        if r[j] == s[k] || table[k] < 0 {
            j += 1;
            k += 1;
        } else {
            k = table[k] as usize;
        }
    }
    j - k 
}

fn kmp_table(s: &[char]) -> Vec<i32> {
    let mut t = vec![0; s.len() + 1];
    let mut cnd = 0;
    t[0] = -1;
    for pos in 1..s.len() {
        if s[pos] == s[cnd as usize] {
            t[pos] = t[cnd as usize];
        } else {
            t[pos] = cnd;
            while cnd >= 0 && s[pos] != s[cnd as usize] {
                cnd = t[cnd as usize];
            }
        }
        cnd += 1;
    }
    t
}
