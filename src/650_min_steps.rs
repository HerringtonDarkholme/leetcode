impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut ret = vec![0];
        for i in 2..=(n as usize) {
            let mut min = 1000;
            for j in 1..=ret.len() {
                if i % j != 0 { continue; }
                min = min.min(ret[j - 1] + i / j);
            }
            ret.push(min);
        }
        ret[ret.len() - 1] as i32
    }
}
