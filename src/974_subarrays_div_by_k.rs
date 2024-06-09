impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut bins = vec![0; k as usize];
        bins[0] = 1;
        let mut sum = 0;
        let mut ret = 0;
        for n in nums {
            sum += n;
            let i = ((sum % k + k) % k) as usize;
            ret += bins[i];
            bins[i] += 1;
        }
        ret
    }
}
