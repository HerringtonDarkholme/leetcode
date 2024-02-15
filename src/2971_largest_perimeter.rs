impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        let mut sum = 0;
        for &n in &nums {
            sum += n as i64;
        }
        for j in (2..nums.len()).rev() {
            if  sum > 2 * nums[j] as i64 {
                return sum;
            }
            sum -= nums[j] as i64;
        }
        -1
    }
}
//   | |
// 1 1 2 3 5 12 50
