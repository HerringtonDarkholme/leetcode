pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
        let len = a.len();
        let mut odds = vec![false; len]; // can reach end with initial odd
        let mut evens = vec![false; len]; // reachable with initial even
        odds[len - 1] = true;
        evens[len - 1] = true;
        let mut tree = BTreeMap::new();
        tree.insert(a[len - 1], len - 1);
        for i in 1..len {
            let index = len - i - 1;
            let elem = a[index];
            // this elem's next odd jump
            let odd = tree.range(elem..)
                .next()
                .map(|k| *k.1)
                .unwrap_or(0);
            // next even jump
            let even = tree.range(..=elem)
                .rev()
                .next()
                .map(|k| *k.1)
                .unwrap_or(0);
            // next odd jump target can only reach end by even jump
            odds[index] = evens[odd];
            // next even jump target can only reach end by odd jump
            evens[index] = odds[even];
            tree.insert(elem, index);
        }
        let mut ret = 0;
        for odd in odds {
            if odd {
                ret += 1;
            }
        }
        ret
    }
}
