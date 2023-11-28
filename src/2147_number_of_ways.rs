const M: i64 = 1_000_000_007;
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut seen = 0;
        let mut last = 0;
        let mut ret = 1;
        for (i, c) in corridor.chars().enumerate() {
            if c == 'S' {
                seen += 1;
                if seen == 2 {
                    last = i;
                }
            }
            if seen == 3 {
                seen = 1;
                ret *= (i - last) as i64;
                ret = ret % M;
            }
        }      
        if seen == 1 || last == 0 { // left one seat or no seat
            0
        } else {
            ret as i32
        }
    }
}
