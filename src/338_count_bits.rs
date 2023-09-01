impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut v = vec![1];
        let mut i = n;
        let mut ret = vec![0, 1];
        while i > 1 {
            let n = v.clone().into_iter().map(|i| i + 1);
            v.extend(n);
            ret.extend(v.iter().copied());
            i /= 2;
        }
        ret[..=n as usize].to_vec()
    }
}
// 0,
// 1,
// 1,2,
// 1,2,2,3,
// 1,2,2,3,2,3,3,4,
// 1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,
// 1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,
