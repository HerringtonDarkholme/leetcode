/*
So this problem is to give two subsequences that has minimum target value.
A brute force solution has exponential complexity 2**n.
But we know we are to assign a sequence of elements to 2 bins so that the target value is minimized. This looks like a dynamic programming.
If we know some min distances to type a word with two fingers, given a new char, what's the new distance?
We can maintain two fingers' previous typed chars. And the next min distance is solely determined by **the last chars two fingers typed respectively.**
This gives rise to dp cache: a matrix representing the last char of two finger.
*/
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<_> = word.chars().collect();
        if word.len() <= 2 {
            return 0;
        }
        // dp: the min distances for one finger last typed i-th char.
        // dp[[0]]: the another finger types nothing
        // dp[j+1] : the another finger types j-th char
        let mut dp = vec![
            compute_dist(word[0], word[1]), 0
        ];
        for i in 2..word.len() {
            let c = word[i];
            let prev = word[i - 1];
            let dist = compute_dist(prev, c);
            let mut next = vec![];
            let mut min = i32::max_value();
            for (j, &d) in dp.iter().enumerate() {
                next.push(d + dist);
                if j == 0 {
                    min = min.min(d);
                } else {
                    min = min.min(d + compute_dist(word[j - 1], c));
                }
            }
            next.push(min);
            dp = next;
        }
        dp.into_iter().fold(i32::max_value(), i32::min)
    }
}

fn compute_dist(c1: char, c2: char) -> i32 {
    let c1 = c1 as i32 - 'A' as i32;
    let c2 = c2 as i32 - 'A' as i32;
    let (x1, y1) = (c1 / 6, c1 % 6);
    let (x2, y2) = (c2 / 6, c2 % 6);
    (x1 - x2).abs() + (y1 - y2).abs()
}
