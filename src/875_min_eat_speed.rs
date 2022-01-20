pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max = piles.iter().map(|&p| p).fold(i32::min_value(), i32::max);
        let min = 1;
        bsearch(&piles, h, min, max)
    }
}

fn bsearch(piles: &Vec<i32>, h: i32, mut min: i32, mut max: i32) -> i32 {
    while min < max {
        let mid = min + (max - min) / 2;
        let hour = compute_hour(piles, mid);
        if hour > h {
            min = mid + 1;
        } else {
            max = mid;
        }
    }
    min
}

#[inline]
fn compute_hour(piles: &Vec<i32>, speed: i32) -> i32 {
    piles.iter().map(|&i| (i + speed - 1) / speed).sum()
}

/*

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;
        while left < right {
            let mid = left + (right - left) / 2;
            if time(&piles, mid) <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
fn time(piles: &[i32], k: i32) -> i32 {
    piles.iter().map(|&p| (p + k - 1) / k).sum()
}

*/
