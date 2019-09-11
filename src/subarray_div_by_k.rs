pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let len = a.len();
        let mut count = vec![0; k as usize];
        let mut sum = 0;
        for i in a {
            sum += (i % k) + k;
            sum = sum % k;
            count[sum as usize] += 1;
        }
        for c in count.iter() {
            ret += c * (c - 1);
        }
        ret / 2 + count[0]
    }
}
