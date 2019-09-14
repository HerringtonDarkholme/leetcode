pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counter = HashMap::new();
        let mut all = 0;
        for i in deck {
            all += 1;
            *counter.entry(i).or_insert(0) += 1;
        }
        let occs: Vec<_> = counter.into_iter().map(|(k, v)| v).collect();
        for x in 2..=all {
            if all % x != 0 {
                continue;
            }
            if occs.iter().all(|&o| o % x == 0) {
                return true
            }
        }
        false
    }
}
/*
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counter = HashMap::new();
        let mut all = 0;
        for i in deck {
            all += 1;
            *counter.entry(i).or_insert(0) += 1;
        }
        let mut occs = counter.values();
        let first = *occs.next().unwrap();
        occs.fold(first, |acc, &i| gcd(acc, i)) >= 2
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 { y } else { gcd(y%x, x) }
}
*/
