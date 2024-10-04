impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut counters = vec![0; 1001];
        let mut sum = 0;
        for &s in &skill {
            counters[s as usize] += 1;
            sum += s as usize;
        }
        let pair = sum * 2 / skill.len();
        let mut ret = 0usize;
        for s in skill {
            let s = s as usize;
            if counters[s] <= 0 { 
                continue;
            }
            counters[s] -= 1;
            if pair <= s || (pair - s) > 1000 || counters[pair - s] <= 0 {
                return -1;
            }
            counters[pair - s] -= 1;
            ret += s * (pair - s);
        }
        ret as i64
    }
}
