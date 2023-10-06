impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![1];
        for i in 2..=(n as usize) {
            let max = (1..i).map(|j| {
                // break int by j nad find the max prod of j
                // j can be unbroken
                dp[j - 1].max(j) * (i - j)
            }).max().unwrap();
            dp.push(max);
        }
        dp[n as usize - 1] as i32
    }
}

/*
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
*/
