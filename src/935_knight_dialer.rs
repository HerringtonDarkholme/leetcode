impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut num = vec![1usize; 10];
        let mut next = vec![0; 10];
        let N = 1_000_000_007;
        for i in 1..n {
            next[0] = num[4] + num[6];
            next[1] = num[6] + num[8];
            next[2] = num[7] + num[9];
            next[3] = num[4] + num[8];
            next[4] = num[3] + num[9] + num[0];
            next[5] = 0;
            next[6] = num[1] + num[7] + num[0];
            next[7] = num[2] + num[6];
            next[8] = num[1] + num[3];
            next[9] = num[2] + num[4];
            for i in 0..10 {
                num[i] = next[i] % N;
            }
        }
        let mut sum = 0;
        for i in num {
            sum = (sum + i) % N;
        }
        sum as i32
    }
}
