impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let mut i = piles.len() / 3;
        let mut ret = 0;
        while i < piles.len() {
            ret += piles[i];
            i += 2;
        }
        ret
    }
}
