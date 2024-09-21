impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ret = vec![];
        for i in 1..10 {
            gen_num(i, n, &mut ret);
        }
        ret
    }
}

fn gen_num(leading: i32, bound: i32, ret: &mut Vec<i32>) {
    if leading > bound { return }
    ret.push(leading);
    for i in 0..=9 {
        gen_num(leading * 10 + i, bound, ret);
    }
}
