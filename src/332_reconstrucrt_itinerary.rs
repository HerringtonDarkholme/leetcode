use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map = build_map(tickets);
        let mut ret = vec![];
        dfs(&mut map, "JFK".into(), &mut ret);
        ret.reverse();
        ret
    }
}

fn build_map(tickets: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for t in tickets {
        map.entry(t[0].clone()).or_insert_with(|| vec![]).push(t[1].clone());
    }
    for (_, v) in map.iter_mut() {
        v.sort();
        v.reverse();
    }
    map
}

fn dfs(map: &mut HashMap<String, Vec<String>>, start: String, ret: &mut Vec<String>) {
    let mut stack = vec![];
    stack.push(start);
    let mut dummy = vec![];
    while !stack.is_empty() {
        while let Some(next) = map.get_mut(stack.last().unwrap()).unwrap_or(&mut dummy).pop() {
            stack.push(next);
        }
        ret.push(stack.pop().unwrap());
    }
}
