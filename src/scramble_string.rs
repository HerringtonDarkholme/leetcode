pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut s1 = s1.chars().collect::<Vec<_>>();
        let mut s2 = s2.chars().collect::<Vec<_>>();
        aux(&s1, &s2)
    }
}


fn aux(c1: &[char], c2: &[char]) -> bool {
    assert_eq!(c1.len(), c2.len());
    if c1 == c2 {
        return true
    }
    let mut map = HashMap::new();
    let mut map_f = HashMap::new();
    let mut map_r = HashMap::new();

    for i in 0..(c1.len()-1) {
        *map.entry(c1[i]).or_insert(0) += 1;
        *map_f.entry(c2[i]).or_insert(0) += 1;
        *map_r.entry(c2[c2.len() - i - 1]).or_insert(0) += 1;
        if map == map_f {
            if aux(&c1[0..=i], &c2[0..=i]) && aux(&c1[i+1..], &c2[i+1..]) {
                return true
            }
        }
        if map == map_r {
            if aux(&c1[0..=i], &c2[(c2.len() - i - 1)..]) && aux(&c1[i+1..], &c2[0..(c2.len() - i - 1)]) {
                return true
            }
        }
    }
    false
}
