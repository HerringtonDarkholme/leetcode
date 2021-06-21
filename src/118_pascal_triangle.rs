impl Solution {
    pub fn generate(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ret = vec![vec![1]];
        for i in 0..n-1 {
            let mut p = &ret[i];
            let mut r = vec![1];
            for j in 0..p.len() - 1 {
                r.push(p[j] + p[j+1]);
            }
            r.push(1);
            ret.push(r);
        }
        ret
    }
}
