impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut time = 0;
        for n in arr.into_iter().skip(1) {
            if n < winner {
                time += 1;
            } else {
                winner = n;
                time = 1;
            }
            if time >= k {
                return winner;
            }
        }
        winner
    }
}
