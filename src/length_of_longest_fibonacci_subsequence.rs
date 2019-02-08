pub struct Solution {}

impl Solution {
    pub fn len_longest_fib_subseq(a: Vec<i32>) -> i32 {
        let len = a.len();
        if len < 3 {
            return 0
        }
        let mut visited = Vec::with_capacity(len * len);
        visited.resize(len*len, false);
        let mut map = std::collections::HashMap::new();
        for (i, &n) in a.iter().enumerate() {
            map.insert(n, i);
        }
        let mut max_len = 0;
        for i in 0..(len-2) {
            for j in (i+1)..(len-1) {
                let mut n1 = i;
                let mut n2 = j;
                let mut current_run = 2;
                while n2 < len {
                    if visited[n1*len + n2] {
                        break;
                    }
                    visited[n1*len + n2] = true;
                    if let Some(&n3) = map.get(&(a[n1] + a[n2])) {
                        current_run += 1;
                        n1 = n2;
                        n2 = n3;
                    } else {
                        break;
                    }
                }
                if current_run > max_len && current_run >= 3 {
                    max_len = current_run;
                }
            }
        }
        max_len
    }
}

#[test]
fn test() {
    let tests = vec![
        (vec![1,2,3,4,5,6,7,8], 5),
        (vec![1,2,3,4,5,6,7,8,9,10,11,12,13], 6),
        (vec![1,3,7,11,12,14,18], 3),
        (vec![], 0),
        (vec![1], 0),
        (vec![1,2], 0),
        (vec![1,2,3], 3),
        (vec![2,2,2], 0),
        (vec![2,3,9,11,13,15,17,19,20,22,24,26,31,32,33,34,37,39], 5),
    ];
    for (case, expect) in tests {
        assert_eq!(expect, Solution::len_longest_fib_subseq(case));
    }
}
