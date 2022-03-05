impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut o = Vec::with_capacity(k as usize);
        let mut ret = vec![];
        helper(n, k as usize, &mut o, &mut ret);
        ret
    }
}

fn helper(n: i32, k: usize, o: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
    if k == 0 {
        ret.push(o.clone());
        return;
    }
    if n == 0 {
        return;
    }
    // pick the element
    o.push(n);
    helper(n - 1, k - 1, o, ret);
    o.pop();
    // not pick
    helper(n - 1, k, o, ret);
}
