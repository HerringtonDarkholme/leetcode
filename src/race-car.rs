pub struct Solution;

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut cache = vec![];
        for i in 0..=target {
            Solution::race(i as usize, &mut cache);
        }
        cache[target as usize] as i32
    }
    fn race(target: usize,  cache:&mut Vec<u32>) -> u32 {
        if cache.len() > target {
            return cache[target]
        }
        let mut a = 0;
        let mut ran = 0usize;
        while ran + 2usize.pow(a) <= target {
            ran += 2usize.pow(a);
            a += 1;
        }
        if ran == target { // e.g 3, 7, 15
            cache.push(a);
        } else if ran == target - 1 { // e.g. 4, 8, 16
            cache.push(a + 3); // R R A
        } else {
            let mut reset = u32::max_value(); // R R AAAAA, R A R AAAA, R AA R AAA
            for i in 0..a {
                let r = 1+i+1 + Solution::race(target - ran + 2usize.pow(i) - 1, cache);
                reset = reset.min(r);
            }
            let over = 1 + 1 + Solution::race(ran + 2usize.pow(a) - target, cache); // A R AAAAAAA
            cache.push(a + reset.min(over));
        }
        return cache[target]
    }
}
