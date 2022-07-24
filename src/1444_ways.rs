use std::collections::HashMap;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let mut dp = HashMap::new();
        let row = pizza.len();
        let col = pizza[0].len();
        let apples = build_apples(pizza);
        ways((0, row - 1, 0, col - 1, k as usize), &apples, &mut dp) as i32

    }
}
type Cuts = (usize, usize, usize, usize, usize);

fn build_apples(pizza: Vec<String>) -> Vec<Vec<usize>> {
    let row = pizza.len();
    let col = pizza[0].len();
    let mut apples = vec![vec![0; col + 1]; row + 1];
    for i in 0..row {
        let bytes = pizza[i].as_bytes();
        for j in 0..col {
            let has_apple = if bytes[j] == b'A' { 1 } else { 0 };
            apples[i+1][j+1] = apples[i + 1][j] + apples[i][j + 1] - apples[i][j] + has_apple;
        }
    }
    apples
}

fn count_apple(cuts: Cuts, apples: &Vec<Vec<usize>>) -> usize {
    let (sr, er, sc, ec, _) = cuts;
    let top_left = apples[sr][sc];
    let bot_right = apples[er + 1][ec + 1];
    let bot_left = apples[er + 1][sc];
    let top_right = apples[sr][ec + 1];
    bot_right + top_left - bot_left - top_right
}

const M: i64 = 1_000_000_007;
fn ways(cuts: Cuts, apples: &Vec<Vec<usize>>, dp: &mut HashMap<Cuts, i64>) -> i64 {
    if let Some(&ret) = dp.get(&cuts) {
        return ret;
    }
    let (sr, er, sc, ec, k) = cuts;
    if count_apple(cuts, apples) < k  {
        dp.insert(cuts, 0);
        return 0;
    }
    if k == 1 {
        return 1;
    }

    let mut ret = 0;
    for h_cut in sr..er {
        if count_apple((sr, h_cut, sc, ec, 1), apples) > 0 {
            ret = (ret + ways((h_cut + 1, er, sc, ec, k - 1), apples, dp)) % M;
        }
    }
    for v_cut in sc..ec {
        if count_apple((sr, er, v_cut, ec, 1), apples) > 0 {
            ret = (ret + ways((sr, er, v_cut + 1, ec, k - 1), apples, dp)) % M;
        }
    }
    dp.insert(cuts, ret);
    ret
}
