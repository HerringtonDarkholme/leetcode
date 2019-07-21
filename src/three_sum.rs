pub struct Solution;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hash: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        let mut ret = BTreeSet::new();
        let mut met: BTreeMap<i32, usize> = BTreeMap::new();
        for i in 0..nums.len() {
            let p = nums[i];
            let m = met.entry(p).or_insert(0);
            if *m >= 2 {
                continue;
            }
            *m += 1;
            for j in (i+1)..nums.len() {
                let q = nums[j];
                hash.entry(-p - q).or_insert(vec![])
                    .push((i, j));
            }
        }
        for i in 0..nums.len() {
            let p = nums[i];
            if hash.get(&p).is_none() {
                continue;
            }
            for (j, k) in hash.get(&p).unwrap() {
                let (j, k) = (*j, *k);
                if i == j || i == k {
                    continue;
                }
                let (q, r) = (nums[j], nums[k]);
                if p + q + r != 0 {
                    continue;
                }
                let (p, q, r) = if p > q {
                    if q > r {
                        (r, q, p)
                    } else if p > r {
                        (q, r, p)
                    } else {
                        (q, p, r)
                    }
                } else {
                    if p > r {
                        (r, p, q)
                    } else if q > r {
                        (p, r, q)
                    } else {
                        (p, q, r)
                    }
                };
                if p + q + r == 0 {
                    ret.insert((p, q, r));
                }
            }
        }
        ret.into_iter().map(|(p, q, r)| vec![p, q, r]).collect()
    }
}
