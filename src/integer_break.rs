pub struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut cache = [1; 59];
        let n = n as usize;
        for i in 3..=n {
            populate(&mut cache, i);
        }
        cache[n] as i32
    }
}

fn populate(cache: &mut [usize], num: usize) {
    let mut max = 1;
    for i in 1..=(num + 1)/2 {
        let n = cache[i].max(i);
        let m = cache[num - i].max(num - i);
        max = max.max(n * m);
    }
    cache[num] = max;
}
