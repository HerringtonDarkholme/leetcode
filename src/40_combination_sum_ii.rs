use std::collections::BTreeMap;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![vec![]]
        }
        let mut map = BTreeMap::new();
        for c in candidates {
            *map.entry(c).or_insert(0) += 1;
        }
        let a = aux(&map, 0, target);
        a.into_iter().map(|mut v| {
            v.reverse();
            v
        }).collect()
    }
}

fn aux(map: &BTreeMap<i32, i32>, start: i32, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for (&k, &v) in map.range(start..=target) {
        for c in 1..=v {
            if c * k == target {
                ret.push(vec![k; c as usize]);
                break;
            } else if c * k > target {
                break;
            } else {
                let n_target = target - c * k;
                if n_target < k+1 {
                    continue;
                }
                let mut temp = aux(map, k+1, n_target);
                for mut t in temp {
                    let l = t.len();
                    t.resize(l + c as usize, k);
                    ret.push(t);
                }
            }
            
        }
    }
    ret
}
