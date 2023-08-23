use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counter = vec![0; 26];
        for c in s.bytes() {
            counter[(c - b'a') as usize] += 1;
        }
        let mut heap: BinaryHeap<_> = counter
            .into_iter()
            .enumerate()
            .filter(|b| b.1 != 0)
            .map(|(c, cnt)| (cnt, c))
            .collect();
        let mut ret = String::new();
        let mut last = heap.pop().unwrap();
        ret.push((last.1 as u8 + b'a') as char);
        last.0 -= 1;
        while !heap.is_empty() {
            let (count, c) = heap.pop().unwrap();
            ret.push((c as u8 + b'a') as char);
            if last.0 > 0 {
                heap.push(last);
            }
            last = (count - 1, c);
        }
        if last.0 > 0 { "".into() } else { ret }
    }
}

pub struct Solution;

use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct Entry {
    ch: char,
    occ: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.occ.cmp(&other.occ) {
            Ordering::Equal => other.ch.cmp(&self.ch),
            o => o,
        }
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let l = s.len();
        if l <= 1 {
            return s;
        }
        let mut hash = std::collections::HashMap::new();
        let mut heap = std::collections::BinaryHeap::new();
        for c in s.chars() {
            *hash.entry(c).or_insert(0) += 1;
        }
        for (ch, occ) in hash.into_iter() {
            heap.push(Entry{ch: ch, occ: occ});
        }
        let mut ret = String::with_capacity(l);
        let mut prev = Entry{ch: '#', occ: -1};
        while !heap.is_empty() {
            let mut k = heap.pop().unwrap();
            ret.push(k.ch);
            if prev.occ > 0 {
                heap.push(prev);
            }
            k.occ -= 1;
            prev = k;
        }
        if ret.len() != l {
            "".to_owned()
        } else {
            ret
        }
    }
}

#[test]
fn test() {
    for i in vec![
        "fdfdfdfdydydydydyzyzyzyzyzyzyzyzylklklklklklklklklklk",


    ] {
        println!("{}", Solution::reorganize_string(i.to_owned()));
    }
}
