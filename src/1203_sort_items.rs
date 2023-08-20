use std::collections::HashSet;

impl Solution {
    pub fn sort_items(n: i32, m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let mut grouped = group_items(m as usize, &mut group);
        if let Some(deps) = sort_within_group_and_get_group_deps(&mut grouped, &group, &before_items) {
            sort_between_groups(grouped, deps)
        } else {
            vec![]
        }
    }
}
// returns a map of group_id => Vec<item_id>
fn group_items(m: usize, groups: &mut Vec<i32>) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![]; m];
    for (num, group) in groups.iter_mut().enumerate() {
        if *group == -1 {
            ret.push(vec![num]);
            *group = ret.len() as i32 - 1;
        } else {
            ret[*group as usize].push(num);
        }
    }
    ret
}

// returns a map of group_id => Vec<before_group>
// it also sort the group array in place
fn sort_within_group_and_get_group_deps(grouped: &mut Vec<Vec<usize>>, group: &[i32], before: &Vec<Vec<i32>>) -> Option<Vec<HashSet<usize>>> {
    let mut deps = vec![HashSet::new(); grouped.len()];
    for (group_id, items) in grouped.iter_mut().enumerate() {
        let mut sorted = vec![];
        let dep = &mut deps[group_id];
        let mut seen = HashSet::new();
        let mut visited = HashSet::new();
        for &item in items.iter() {
            if insert(&mut sorted, item, group, before, dep, &mut visited, &mut seen) {
                return None
            }
        }
        *items = sorted;
    }
    Some(deps)
}
// returns if has cycle
fn insert(sorted: &mut Vec<usize>, item: usize, group: &[i32], before: &Vec<Vec<i32>>, dep: &mut HashSet<usize>, visited: &mut HashSet<usize>, seen: &mut HashSet<usize>) -> bool {
    if visited.contains(&item) {
        return false;
    }
    if seen.contains(&item) {
        return true;
    }
    seen.insert(item);
    for &b in &before[item] {
        let b = b as usize;
        if group[b] != group[item] {
            // not in same group
            dep.insert(group[b] as usize);
        } else if insert(sorted, b, group, before, dep, visited, seen) {
            // in the same group
            return true;
        }
    }
    sorted.push(item);
    visited.insert(item);
    false
}

fn sort_between_groups(grouped: Vec<Vec<usize>>, deps: Vec<HashSet<usize>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut visited = HashSet::new();
    let mut seen = HashSet::new();
    for group_id in 0..grouped.len() {
        if insert_group(&mut ret, group_id, &grouped, &deps, &mut visited, &mut seen) {
            return vec![]
        }
    }
    ret
}
// returns if has cycle
fn insert_group(ret: &mut Vec<i32>, group_id: usize, grouped: &Vec<Vec<usize>>, deps: &Vec<HashSet<usize>>, visited: &mut HashSet<usize>, seen: &mut HashSet<usize>) -> bool {
    if visited.contains(&group_id) {
        return false;
    }
    if seen.contains(&group_id) {
        return true;
    }
    seen.insert(group_id);
    for &before in &deps[group_id] {
        if insert_group(ret, before, grouped, deps, visited, seen) {
            return true;
        }
    }
    visited.insert(group_id);
    for &item in &grouped[group_id] {
        ret.push(item as i32);
    }
    false
}
