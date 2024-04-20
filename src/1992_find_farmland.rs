impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for r in 0..land.len() {
            for c in 0..land[0].len() {
                if let Some((i, j)) = dfs(r, c, &mut land) {
                    ret.push(vec![r as i32, c as i32, i as i32, j as i32,]);
                }
            }
        }
        ret
    }
}
fn dfs(r: usize, c: usize, land: &mut Vec<Vec<i32>>) -> Option<(usize, usize)> {
    if land[r][c] == 0 { return None }
    let (mut row, mut col) = (0, 0);
    for i in r..land.len() {
        if land[i][c] == 0 { break; }
        row = i;
        for j in c..land[0].len() {
            if land[i][j] == 0 { break; }
            land[i][j] = 0;
            col = j;
        }
    }
    Some((row, col))
}
