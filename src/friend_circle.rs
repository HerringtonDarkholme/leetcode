pub struct Solution;

impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let mut ret = 0;
        for i in 0..m.len() {
            if seen.contains(&i) {
                continue;
            }
            ret +=1;
            seen.insert(i);
            let mut stack = vec![i];
            while !stack.is_empty() {
                let j = stack.pop().unwrap();
                for (k, &f) in m[j].iter().enumerate() {
                    if seen.contains(&k) || f == 0 {
                        continue;
                    }
                    seen.insert(k);
                    stack.push(k);
                }

            }
        }
        ret
    }
}
