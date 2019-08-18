pub struct Solution;

impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let mut i = 0;
        for aj in a.iter() { // remove j's boundary check
            k += aj - 1;
            if k < 0 {
                k += a[i] ^ 1;
                i += 1;
            }
        }
        (a.len() - i) as i32
    }
}

/*
impl Solution {
    pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut zeros: Vec<_> = a.iter().enumerate()
            .filter_map(|(p, &i)| if i == 0 { Some(p) } else { None })
            .collect();
        if zeros.len() <= k {
            return a.len() as i32
        }
        zeros.push(a.len());
        let end = zeros.len() - k;
        let mut max = 0;
        let mut last_pos = 0;
        for i in 0..end {
            max = max.max(zeros[i + k] - last_pos);
            last_pos = zeros[i] + 1;
        }
        max as i32
    }
}
*/

#[test]
fn test() {
    for t in vec![
        (vec![1,1,1,0,0,0,1,1,1,1,0], 2, 6),
        (vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3, 10),
        (vec![1,0,1,0,1,0], 3, 6),
        (vec![1,1,1,0,0,0,1,1,1,1,0,0,0,1,1,0,1,0,1,0,1,0,1,0], 5, 13),
        (vec![0,0,0,0,1,1,1,1,1,1,1,1,1,0,0], 3, 12),
    ] {
        assert_eq!(Solution::longest_ones(t.0, t.1), t.2);
    }
}
