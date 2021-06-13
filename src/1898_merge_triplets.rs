impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = vec![false; target.len()];
        for t in triplets {
            let has_exceed = (0..t.len()).any(|i| t[i] > target[i]);
            if has_exceed {
                continue;
            }
            for i in 0..t.len() {
                if t[i] == target[i] {
                    found[i] = true;
                }
            }
        }
        found.into_iter().all(|b| b)
    }
}
