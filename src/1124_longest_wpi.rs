use std::collections::HashMap;
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut previous_diff = HashMap::new();
        previous_diff.insert(0, 0);
        let mut diff = 0;
        let mut longest = 0;
        for (i, hour) in hours.into_iter().enumerate() {
            diff += if hour > 8 { 1 } else { -1 };
            if diff > 0 {
                longest = longest.max(i + 1);
                continue;
            }
            if let Some(&p) = previous_diff.get(&(diff - 1)) {
                longest = longest.max(i - p);
            } else if !previous_diff.contains_key(&diff) {
                previous_diff.insert(diff, i);
            }
        }
        longest as i32
    }
}
// TTFFFFT
// we need count the diff of well-day vs bad-day
// so assign well-day as 1, bad-day as -1, counting diff is equivalent to count the sum of interval
// using perfix sum can help us find intervals of positive sum
// if the sum on i-th day is n, and n > 0, longest is just n
// else find the farthest j-th day of which the sum is n-1
// their diff is the longest interval ending // on i-th day

//  num: 0 T T F F T F F T
// diff: 0 1 2 1 0
