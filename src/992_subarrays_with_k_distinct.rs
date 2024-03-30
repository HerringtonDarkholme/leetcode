pub struct Solution;
impl Solution {
    // sliding window can maintain how many distincts in the longest subarray
    // however, count all subarray number ends at the latter pointer would require n time traverse
    // e.g. for array [1 2 1 2], suppose sliding window ends at 2, and k = 2
    // [ 1  2   1   2
    //   ↑      ↑-- end of sliding window
    //   |------- start of sliding window
    // how many subarrays are within the window? there are two, starting at 0 and 1 respectively
    // which are [1, 2, 1] and [2, 1]. we can traverse from the start of window to the point where
    // there are not enough distinct element, which is, the start of subarray with just  k - 1 elements
    // so we can maintian two sliding windows, for each position in array, the possible number of subarrays
    // is the difference of the two sliding window start.
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut just_k = MultiSet::new();
        let mut miss_1 = MultiSet::new();
        let mut just = 0;
        let mut miss = 0;
        let mut ret = 0;
        for i in 0..a.len() {
            just_k.add(a[i]);
            miss_1.add(a[i]);
            while just_k.size() > k {
                just_k.remove(&a[just]);
                just += 1;
            }
            while miss_1.size() > k - 1 {
                miss_1.remove(&a[miss]);
                miss += 1;
            }
            if just_k.size() == k {
                ret += miss - just;
            }
        }
        ret as i32
    }
}

use std::hash::Hash;
use std::borrow::Borrow;
use std::collections::HashMap;

struct MultiSet<T> where T: Hash + Eq {
    inner: HashMap<T, usize>
}

impl<T> MultiSet<T> where T: Hash + Eq {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    fn add(&mut self, t: T) {
        *self.inner.entry(t).or_insert(0) += 1;
    }
    fn remove<Q>(&mut self, t: &Q) where
        T: Borrow<Q>, Q: Hash + Eq
    {
        *self.inner.get_mut(t).unwrap() -= 1;
        if self.inner[t] == 0 {
            self.inner.remove(t);
        }
    }
    fn size(&self) -> usize {
        self.inner.len()
    }
}


// the below solution actually runs faster LOL
/*
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut map = HashMap::new();
        let mut ret = 0;
        let k = k as usize;
        while end <= a.len() {
            // fill map
            while end < a.len() && map.len() < k {
                *map.entry(a[end]).or_insert(0) += 1;
                end += 1;
            }
            if map.len() < k {
                break;
            }
            let initial_e = end;
            end -= 1;
            while end < a.len() && map.contains_key(&a[end]) {
                ret += 1;
                end += 1;
            }
            *map.get_mut(&a[start]).unwrap() -= 1;
            if map[&a[start]] == 0 {
                map.remove(&a[start]);
            }
            start += 1;
            end = initial_e;
        }
        ret
    }
}
 *
 * */
