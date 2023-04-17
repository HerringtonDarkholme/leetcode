const M: usize = 1_000_000_007;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let w_len = words[0].len();
        let t_len = target.len();
        let mut dp = vec![vec![0; w_len]; t_len];
        let map = build_map(words);
        let target = target.as_bytes();
        let c = (target[0] - b'a') as usize;
        // f(0, 0) = map[0][c]
        dp[0][0] = map[0][c];
        // f(0, j) = map[j][c] + dp[0][j - 1]
        for j in 1..w_len {
            dp[0][j] = (map[j][c] + dp[0][j-1]) %  M;
        }
        // f(i, j) = map[j][c] * f(i - 1, j - 1) + f(i, j-1)
        for i in 1..t_len {
            let c = (target[i] - b'a') as usize;
            for j in 1..w_len {
                dp[i][j] = (map[j][c] * dp[i - 1][j - 1] % M + dp[i][j - 1] % M) % M;
            }
        }
        dp[t_len - 1][w_len - 1] as i32
    }
}

fn build_map(words: Vec<String>) -> Vec<Vec<usize>> {
    let mut map = vec![vec![0; 26]; words[0].len()];
    for word in words {
        for (i, b) in word.bytes().enumerate() {
            let c = (b - b'a') as usize;
            map[i][c] += 1;
        }
    }
    map
}

/*
a b a
f(0, 0) = map[0][c]
f(0, j) = map[j][c] + dp[0][j - 1]
f(i, j) = map[j][c] * f(i - 1, j - 1) + f(i, j-1)

words
a c c a 
b b b b
c a c a
=> 
map:
0: a->1, b->1, c->1
1: a->1, b->1, c->1
2: c->2, b->1
3: a->2, b->1

*/
