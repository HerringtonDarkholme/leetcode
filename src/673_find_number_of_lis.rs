use std::collections::HashMap;
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut seen = HashMap::new();
        let mut max = 0;
        for n in nums {
            let mut len = 1;
            let mut cnt = 1;
            for (&m, &(l, c)) in seen.iter() {
                if m > n { continue; }
                let o = if m == n { 0 } else { 1 };
                if l + o > len {
                    len = l + o;
                    cnt = c;
                } else if l + o == len {
                    cnt += c;
                }
            }
            max = max.max(len);
            seen.insert(n, (len, cnt));
        }
        seen.values().filter_map(|&(l, c)| {
            if l == max {
                Some(c)
            } else {
                None
            }
        }).sum()
        
    }
}
