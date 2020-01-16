impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut low = 1;
        let mut high = i32::max_value();
        while low < high {
            let mid = low + (high - low) / 2;
            if sum(&nums, mid) > threshold {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}
fn divide(a: i32, d: i32) -> i32 {
    a / d + if a % d != 0 { 1 } else { 0 } 
}

fn sum(nums: &Vec<i32>, divisor: i32) -> i32 {
    nums.iter().map(|&a| divide(a, divisor)).sum()    
}
