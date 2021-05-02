impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        max_building(n, restrictions)
    }
}

fn max_building(n: i32, mut res: Vec<Vec<i32>>) -> i32 {
    res.push(vec![1, 0]);
    res.sort_by_key(|v| v[0]);
    for i in 1..res.len() {
        let next = &res[i];
        let prev = &res[i - 1];
        if next[1] - prev[1] > next[0] - prev[0] {
            res[i][1] = 0.max(prev[1] + next[0] - prev[0]);
        }
    }
    res.reverse();
    for i in 1..res.len() {
        let next = &res[i];
        let prev = &res[i - 1];
        if next[1] - prev[1] > prev[0] - next[0] {
            res[i][1] = 0.max(prev[1] + prev[0] - next[0]);
        }
    }
    let mut ret = 0;
    for i in 1..res.len() {
        let next = &res[i];
        let prev = &res[i - 1];
        let max = prev[1].max((prev[0] - next[0] + next[1] + prev[1]) / 2 );
        ret = ret.max(max);
    }
    if res[0][0] != n {
        ret = ret.max(res[0][1] + n - res[0][0]);
    }
    ret
}
