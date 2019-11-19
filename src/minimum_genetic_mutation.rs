// leetcode 433
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if start == end {
            return 0
        }
        let len = bank.len();
        let start = start.chars().collect::<Vec<_>>();
        let end = end.chars().collect::<Vec<_>>();
        let bank = bank.iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
        let graph = build_graph(&bank);
        let mut visited = vec![false; len];
        let mut stack: Vec<_> = bank.iter().enumerate().filter_map(|(i, s)| {
            if is_one_off(s, &start) {
                visited[i] = true;
                Some(i)
            } else {
                None
            }
        }).collect();
        let mut ret = 1;
        while !stack.is_empty() {
            let mut next = vec![];
            for i in stack {
                let gene = &bank[i];
                if gene == &end {
                    return ret;
                }
                for j in 0..len {
                    if graph[i][j] && !visited[j] {
                        visited[j] = true;
                        next.push(j);
                    }
                }
            }
            stack =next;
            ret += 1;
        }
        -1
    }
}

fn is_one_off(s1: &[char], s2: &[char]) -> bool {
    let mut has_change = false;
    for i in 0..8 {
        if s1[i] != s2[i] {
            if has_change {
                return false
            }
            has_change = true;
        }
    }
    true
}

fn build_graph(bank: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let len = bank.len();
    let mut graph = vec![vec![false; len]; len];
    for i in 0..len {
        for j in i..len {
            if is_one_off(&bank[i], &bank[j]) {
                graph[i][j] = true;
                graph[j][i] = true;
            }
        }
    }
    graph
}
