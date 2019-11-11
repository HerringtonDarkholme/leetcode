pub struct Solution;

impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        a.sort();
        let mut b: Vec<_> = b.into_iter().enumerate().collect();
        b.sort_by_key(|v| v.1);
        let mut ret = vec![0; b.len()];
        let mut i = 0;
        let mut j = a.len() - 1;
        // tian ji sai ma
        for &(k, v) in b.iter().rev() {
            if a[j] > v {
                ret[k] = a[j];
                j -= 1;
            } else {
                ret[k] = a[i];
                i += 1;
            }
        }
        ret
    }
}
