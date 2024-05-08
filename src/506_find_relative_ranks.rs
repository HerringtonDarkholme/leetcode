impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut v: Vec<_> = score.into_iter().enumerate().collect();
        v.sort_unstable_by_key(|n| -n.1);
        let mut ret = vec![String::new(); v.len()];
        for (i, (place, _)) in v.into_iter().enumerate() {
            ret[place] = match i {
                0 => "Gold Medal".into(),
                1 => "Silver Medal".into(),
                2 => "Bronze Medal".into(),
                j => (j + 1).to_string(),
            };
        }
        ret
    }
}
