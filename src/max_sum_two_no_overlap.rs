pub struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        let l = l as usize;
        let m = m as usize;
        let mut prefix_sums = vec![0];
        let mut sum = 0;
        for i in a {
            sum += i;
            prefix_sums.push(sum);
        }
        let len = prefix_sums.len();
        let mut l_max = prefix_sums[l]; // largest len l array before current - m
        let mut m_max = prefix_sums[m]; // largest len m array before current - l
        let mut max = 0;
        for i in (l + m)..len {
            l_max = l_max.max(prefix_sums[i - m] - prefix_sums[i - m - l]); // keep last m elements
            m_max = m_max.max(prefix_sums[i - l] - prefix_sums[i - l - m]); // keep last l elements
            let lm = l_max + prefix_sums[i] - prefix_sums[i - m]; // previous l_max + current m
            let ml = m_max + prefix_sums[i] - prefix_sums[i - l]; // previous m_max + current l
            max = max.max(lm.max(ml));
        }
        max
    }
}
