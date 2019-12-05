// leetcode 1103
pub struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num: i32) -> Vec<i32> {
        let mut round = 0;
        let mut ret = vec![0; num as usize];
        while candies >= (2 * round * num + num + 1) * num / 2 {
            candies -= (2 * round * num + num + 1) * num / 2;
            round += 1;
        }
        for i in 1..=num {
            ret[i as usize - 1] = (i + (round - 1) * num + i) * round / 2;
        }
        let mut start = round * num + 1;
        for i in 0..num as usize {
            if candies == 0 {
                break;
            }
            ret[i] += candies.min(start);
            candies = 0.max(candies - start);
            start += 1;
        }
        ret
    }
}
