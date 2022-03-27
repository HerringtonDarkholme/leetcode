// 101
// 1000001 -> 1..=10
// 1010101 ->
impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        queries.into_iter().map(|nth| build_palindrome(nth as i64 - 1, int_length as usize)).collect()
    }
}

fn build_palindrome(mut n_th: i64, len: usize) -> i64 {
    let mut nums = vec![0i64; len];
    let max_span = len / 2;
    let mut left = (len - 1) / 2;
    let mut right = len / 2;
    loop {
        if left == 0 {
            if n_th >= 9 {
                return -1;
            }
            nums[left] = n_th % 10 + 1;
            nums[right] = n_th % 10 + 1;
            n_th /= 10;
            break;
        }
        nums[left] = n_th % 10;
        nums[right] = n_th % 10;
        left -= 1;
        right += 1;
        n_th /= 10;
    }
    if n_th > 0 {
        -1
    } else {
        nums.into_iter().fold(0, |acc, n| acc * 10 + n)
    }
}
