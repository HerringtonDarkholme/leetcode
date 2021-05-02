use std::collections::{BTreeMap, HashMap};

struct TopVotedCandidate {
    map: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {

    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut votes = HashMap::new();
        let mut map = BTreeMap::new();
        let mut max = 0;
        for (&p, &t) in persons.iter().zip(times.iter()) {
            let m = votes.entry(p).or_insert(0);
            *m += 1;
            max = max.max(*m);
            if *m == max {
                map.insert(t, p);
            }
        }
        TopVotedCandidate{
            map
        }
    }
    
    fn q(&self, t: i32) -> i32 {
        self.map
            .range(..=t)
            .rev()
            .next()
            .map(|c| *c.1)
            .expect("t >= times[0]")
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
