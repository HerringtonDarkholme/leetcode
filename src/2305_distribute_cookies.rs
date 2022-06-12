impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut ret = i32::MAX;
        let mut pockets = vec![0; k as usize];
        dfs(&cookies, 0, &mut &mut pockets, &mut ret);
        ret
    }
}

fn dfs(cookies: &[i32], curr: usize, pockets: &mut [i32], ret: &mut i32) {
    if curr == cookies.len() {
        let max = *pockets.iter().max().unwrap();
        if *ret > max  {
            *ret = max;
        }
        return;
    }
    for i in 0..pockets.len() {
        pockets[i] += cookies[curr];
        dfs(cookies, curr + 1,  pockets, ret);
        pockets[i] -= cookies[curr];
    }
}


