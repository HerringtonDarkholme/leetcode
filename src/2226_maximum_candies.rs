impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 0;
        let mut right = find_max(&candies, k);
        while left < right {
            let mid = left + (right - left) / 2;
            if can_divide(&candies, mid, k) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if can_divide(&candies, left, k) {
            left
        } else {
            left - 1
        }
    }
}
fn find_max(candies: &[i32], k: i64) -> i32 {
    let mut sum = 0;
    for &c in candies {
        sum += c as i64;
    }
    (sum / k) as i32
}

fn can_divide(candies: &[i32], each: i32, k: i64) -> bool {
    if each == 0 {
        return true;
    }
    let mut kids = 0;
    for &c in candies {
        kids += (c / each) as i64;
    }
    kids >= k
}

