// leetcode 646
pub Solution;

// greedy solution
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|v| v[1]);
        let mut ret = 1;
        // sort by last element. thus the end element of Longest chain
        // is the smallest and can be readily followed
        let mut last = pairs[0][1];
        for p in pairs {
            if last < p[0] {
                ret += 1;
                last = p[1];
            }
        }
        ret
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
