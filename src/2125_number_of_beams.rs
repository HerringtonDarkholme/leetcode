impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last = 0;
        let mut ret = 0;
        for row in bank {
            let cnt = row.chars().filter(|&c| c == '1').count();
            if cnt > 0 {
                ret += last * cnt;
                last = cnt;
            }
        }
        ret as i32
    }
}
