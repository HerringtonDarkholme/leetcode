impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut parents: Vec<_> = (0..s.len()).collect();
        for pair in pairs {
            union(&mut parents, pair[0] as usize, pair[1] as usize);
        }
        let mut groups = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let group = find(&parents, i);
            groups.entry(group).or_insert_with(|| vec![]).push(c);
        }
        for (_, group) in groups.iter_mut() {
            group.sort();
            group.reverse();
        }
        let mut ret = String::new();
        for i in 0..s.len() {
            let group = find(&parents, i);
            ret.push(groups.get_mut(&group).unwrap().pop().unwrap());
        }
        ret
    }
}

fn union(parents: &mut [usize], x: usize, y: usize) {
    let px = find(parents, x);
    let py = find(parents, y);
    if px < py {
        parents[py] = px;
    } else {
        parents[px] = py;
    }
}

fn find(parents: &[usize], mut x: usize) -> usize {
    while x != parents[x] {
        x = parents[x];
    }
    x
}
