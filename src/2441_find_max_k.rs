impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![false; 2001];
        for n in nums {
            let i = (n + 1000) as usize;
            cnt[i] = true;
        }
        for i in 0..1000 {
            if cnt[i] && cnt[2000 - i] {
                return 1000 - i as i32
            }
        }
        -1
    }
}
