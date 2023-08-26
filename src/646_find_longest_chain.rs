// leetcode 646
pub Solution;

// greedy solution
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| (p[1], -p[0]));
        let mut len = 1;
        let mut last = pairs[0][1];
        for pair in pairs.into_iter().skip(1) {
            if last < pair[0] {
                len += 1;
                last = pair[1];
            }
        }
        len
    }
}


/*
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort();
        let len = pairs.len();
        let mut dp = vec![1; len];
        for i in 0..len {
            let pair1 = &pairs[i];
            let mut max = 1;
            for j in 0..i {
                let pair2 = &pairs[j];
                if pair2[1] < pair1[0] {
                    max = max.max(dp[j] + 1);
                }
            }
            dp[i] = max;
        }
        dp.into_iter().fold(0, i32::max)
    }
}
*/
