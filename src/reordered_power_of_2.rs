pub struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let (bit, map) = get_digits(n);
        let mut m = 1;
        loop {
            let (b, m2) = get_digits(m);
            if b == bit && is_same(&map, &m2) {
                break true
            } else if b > bit {
                break false
            }
            m = m << 1;
        }
    }
}

fn get_digits(mut n: i32) -> (usize, Vec<i32>) {
    let mut bit = 0;
    let mut map = vec![0; 10];
    while n != 0 {
        bit += 1;
        let num = n % 10;
        map[num as usize] += 1;
        n = n / 10;
    }
    (bit, map)
}

fn is_same(m1: &Vec<i32>, m2: &Vec<i32>) -> bool {
    for i in 0..10 {
        if m1[i] != m2[i] {
            return false
        }
    }
    true
}
