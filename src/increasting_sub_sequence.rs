pub struct Solution;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cache = vec![vec![]; nums.len()];
        find_is(&nums, &mut cache);
        let mut seen = std::collections::HashSet::new();
        for vs in cache {
            for v in vs {
                seen.insert(v);
            }
        }
        seen.into_iter().collect()
    }
}

fn find_is(nums: &Vec<i32>, cache: &mut Vec<Vec<Vec<i32>>>) {
    for i in 0..nums.len() {
        let n_end = nums[i];
        for j in 0..i {
            let n_start = nums[j];
            if n_start <= n_end {
                cache[i].push(vec![n_start, n_end]);
                let vs = cache[j].clone();
                for mut v in vs {
                    v.push(n_end);
                    cache[i].push(v);
                }
            }
        }
    }
}
