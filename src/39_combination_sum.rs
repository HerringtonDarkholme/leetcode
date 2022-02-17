impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        comb(&candidates, target)
    }
}

fn comb(cand: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for (i, &n) in cand.iter().enumerate() {
        let mut t = target;
        let mut v = vec![];
        while t >= n {
            t -= n;
            v.push(n);
            if t == 0 {
                ret.push(v);
                break;
            }
            for vv in comb(&cand[i+1..], t) {
                let mut v = v.clone();
                v.extend(vv);
                ret.push(v);
            }
        }
    }
    ret
}
