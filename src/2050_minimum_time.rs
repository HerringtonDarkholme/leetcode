impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut hours = vec![0; n as usize];
        let mut rel = vec![vec![]; n as usize];
        for r in relations {
            rel[r[1] as usize - 1].push(r[0] as usize - 1);
        }
        (0..n as usize).map(|c| {
            dfs(c, &mut hours, &rel, &time)
        }).max().unwrap_or(0)
    }
}

fn dfs(c: usize, hours: &mut Vec<i32>, relations: &Vec<Vec<usize>>, time: &[i32]) -> i32 {
    if hours[c] > 0 {
        return hours[c];
    }
    let precede = relations[c].iter().map(|&pre| {
        dfs(pre, hours, relations, time)
    }).max().unwrap_or(0);
    hours[c] = precede + time[c];
    hours[c]
}
// 3 -> 5 + 3 
// 2 -> 2 + 0
// 1 -> 3 + 0
